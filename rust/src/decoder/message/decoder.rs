//! Veriform message decoder

use super::{hasher::Hasher, state::State};
use crate::{
    decoder::{Decodable, Event},
    error::Error,
    field::{Header, Tag, WireType},
    message::Element,
};
use core::fmt::{self, Debug};
use digest::{generic_array::GenericArray, Digest};

/// Veriform message decoder: streaming zero-copy pull parser which emits
/// events based on incoming data.
pub struct Decoder<D: Digest> {
    /// Last field tag that was decoded (to ensure monotonicity)
    last_tag: Option<Tag>,

    /// Current position within the message (i.e. total bytes consumed)
    position: usize,

    /// Current state of the decoder (or `None` if an error occurred)
    state: Option<State>,

    /// Verihash message hasher
    hasher: Option<Hasher<D>>,
}

impl<D> Decoder<D>
where
    D: Digest,
{
    /// Create a new decoder in an initial state
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the tag (i.e. ID) of the last decoded field header
    pub fn last_tag(&self) -> Option<Tag> {
        self.last_tag
    }

    /// Get the current position (i.e. number of bytes processed) in the
    /// message being decoded
    pub fn position(&self) -> usize {
        self.position
    }

    /// Decode an expected field header, returning an error for anything else
    pub fn decode_header(&mut self, input: &mut &[u8]) -> Result<Header, Error> {
        match self.decode(input)? {
            Some(Event::FieldHeader(header)) => Ok(header),
            _ => Err(Error::FieldHeader {
                tag: None,
                wire_type: None,
            }),
        }
    }

    /// Decode an expected field header, skipping (in-order) unknown fields,
    /// and returning an error if the field is missing or unexpected
    pub fn expect_header(
        &mut self,
        input: &mut &[u8],
        tag: Tag,
        wire_type: WireType,
    ) -> Result<(), Error> {
        let header = self.decode_header(input).map_err(|e| match e {
            Error::FieldHeader { .. } => Error::FieldHeader {
                tag: Some(tag),
                wire_type: Some(wire_type),
            },
            _ => unreachable!("unexpected decode_header error: {:?}", e),
        })?;

        // TODO(tarcieri): actually skip unknown fields
        if header.tag != tag {
            return Err(Error::Decode {
                element: Element::Tag,
                wire_type,
            });
        }

        if header.wire_type != wire_type {
            return Err(Error::UnexpectedWireType {
                actual: header.wire_type,
                wanted: wire_type,
            });
        }

        Ok(())
    }

    /// Hash a digest of a nested message within this message
    pub fn hash_message_digest(
        &mut self,
        tag: Tag,
        digest: &GenericArray<u8, D::OutputSize>,
    ) -> Result<(), Error> {
        if let Some(hasher) = &mut self.hasher {
            hasher.hash_message_digest(tag, digest)?;
        }

        Ok(())
    }

    /// Finish producing a digest of a message, if we're configured to hash.
    ///
    /// Panics if the hasher is in a bad state.
    pub fn finish_digest(self) -> Option<GenericArray<u8, D::OutputSize>> {
        self.hasher.map(|hasher| hasher.finish().unwrap())
    }

    /// Decode a length delimiter, expecting the given wire type
    fn decode_length_delimiter(
        &mut self,
        input: &mut &[u8],
        expected_type: WireType,
    ) -> Result<usize, Error> {
        debug_assert!(
            expected_type.is_dynamically_sized(),
            "not a dynamically sized wire type: {:?}",
            expected_type
        );

        match self.decode(input)? {
            Some(Event::LengthDelimiter { wire_type, length }) if wire_type == expected_type => {
                Ok(length)
            }
            _ => Err(Error::Decode {
                element: Element::LengthDelimiter,
                wire_type: expected_type,
            }),
        }
    }
}

impl<D> Default for Decoder<D>
where
    D: Digest,
{
    fn default() -> Self {
        Self {
            state: Some(State::default()),
            last_tag: None,
            position: 0,
            hasher: Some(Hasher::new()), // TODO(tarcieri): support for disabling hasher
        }
    }
}

impl<D> Decodable for Decoder<D>
where
    D: Digest,
{
    fn decode<'a>(&mut self, input: &mut &'a [u8]) -> Result<Option<Event<'a>>, Error> {
        if let Some(state) = self.state.take() {
            let (new_state, event) = state.decode(input, self.last_tag)?;

            if let Some(Event::FieldHeader(header)) = &event {
                self.last_tag = Some(header.tag);
            }

            self.state = Some(new_state);
            self.position = self.position.checked_add(input.len()).unwrap();

            if let Some(ev) = &event {
                if let Some(hasher) = &mut self.hasher {
                    hasher.hash_event(ev)?;
                }
            }

            Ok(event)
        } else {
            Err(Error::Failed)
        }
    }

    fn decode_dynamically_sized_value<'a>(
        &mut self,
        expected_type: WireType,
        input: &mut &'a [u8],
    ) -> Result<&'a [u8], Error> {
        let length = self.decode_length_delimiter(input, expected_type)?;

        match self.decode(input)? {
            Some(Event::ValueChunk {
                wire_type,
                bytes,
                remaining,
            }) if wire_type == expected_type => {
                if remaining == 0 {
                    debug_assert_eq!(length, bytes.len());
                    Ok(bytes)
                } else {
                    Err(Error::Truncated {
                        remaining,
                        wire_type,
                    })
                }
            }
            _ => Err(Error::Decode {
                element: Element::Value,
                wire_type: expected_type,
            }),
        }
    }
}

impl<D> Debug for Decoder<D>
where
    D: Digest,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Decoder")
            .field("last_tag", &self.last_tag)
            .field("position", &self.position)
            .field("state", &self.state)
            .field("hasher", &self.hasher)
            .finish()
    }
}

#[cfg(all(test, feature = "sha2"))]
mod tests {
    use super::{Decodable, WireType};
    use crate::error::Error;

    type Decoder = super::Decoder<sha2::Sha256>;

    #[test]
    fn decode_false() {
        let input = [130, 10];
        let mut input_ref = &input[..];
        let mut decoder = Decoder::new();

        let header = decoder.decode_header(&mut input_ref).unwrap();
        assert_eq!(header.tag, 42);
        assert_eq!(header.wire_type, WireType::False);
    }

    #[test]
    fn decode_true() {
        let input = [198, 10];
        let mut input_ref = &input[..];
        let mut decoder = Decoder::new();

        let header = decoder.decode_header(&mut input_ref).unwrap();
        assert_eq!(header.tag, 43);
        assert_eq!(header.wire_type, WireType::True);
    }

    #[test]
    fn decode_uint64() {
        let input = [138, 10, 85];
        let mut input_ref = &input[..];
        let mut decoder = Decoder::new();

        let header = decoder.decode_header(&mut input_ref).unwrap();
        assert_eq!(header.tag, 42);
        assert_eq!(header.wire_type, WireType::UInt64);

        let value = decoder.decode_uint64(&mut input_ref).unwrap();
        assert_eq!(value, 42);
        assert!(input_ref.is_empty());
    }

    #[test]
    fn decode_sint64() {
        let input = [206, 10, 167];
        let mut input_ref = &input[..];
        let mut decoder = Decoder::new();

        let header = decoder.decode_header(&mut input_ref).unwrap();
        assert_eq!(header.tag, 43);
        assert_eq!(header.wire_type, WireType::SInt64);

        let value = decoder.decode_sint64(&mut input_ref).unwrap();
        assert_eq!(value, -42);
        assert!(input_ref.is_empty());
    }

    #[test]
    fn decode_bytes() {
        let input = [73, 11, 98, 121, 116, 101, 115];
        let mut input_ref = &input[..];
        let mut decoder = Decoder::new();

        let header = decoder.decode_header(&mut input_ref).unwrap();
        assert_eq!(header.tag, 2);
        assert_eq!(header.wire_type, WireType::Bytes);

        let bytes = decoder.decode_bytes(&mut input_ref).unwrap();
        assert_eq!(bytes, &[98, 121, 116, 101, 115]);
        assert!(input_ref.is_empty());
    }

    #[test]
    fn decode_string() {
        let input = [139, 7, 98, 97, 122];
        let mut input_ref = &input[..];
        let mut decoder = Decoder::new();

        let header = decoder.decode_header(&mut input_ref).unwrap();
        assert_eq!(header.tag, 4);
        assert_eq!(header.wire_type, WireType::String);

        let string = decoder.decode_string(&mut input_ref).unwrap();
        assert_eq!(string, "baz");
        assert!(input_ref.is_empty());
    }

    #[test]
    fn decode_message() {
        let input = [45, 5, 69, 7];
        let mut input_ref = &input[..];
        let mut decoder = Decoder::new();

        let header = decoder.decode_header(&mut input_ref).unwrap();
        assert_eq!(header.tag, 1);
        assert_eq!(header.wire_type, WireType::Message);

        let message = decoder.decode_message(&mut input_ref).unwrap();
        assert_eq!(message, &[69, 7]);
        assert!(input_ref.is_empty());
    }

    #[test]
    fn decode_multiple() {
        let input = [138, 10, 85, 206, 10, 167];
        let mut input_ref = &input[..];
        let mut decoder = Decoder::new();

        let header = decoder.decode_header(&mut input_ref).unwrap();
        assert_eq!(header.tag, 42);
        assert_eq!(header.wire_type, WireType::UInt64);

        let value = decoder.decode_uint64(&mut input_ref).unwrap();
        assert_eq!(value, 42);

        let header = decoder.decode_header(&mut input_ref).unwrap();
        assert_eq!(header.tag, 43);
        assert_eq!(header.wire_type, WireType::SInt64);

        let value = decoder.decode_sint64(&mut input_ref).unwrap();
        assert_eq!(value, -42);
        assert!(input_ref.is_empty());
    }

    #[test]
    fn decode_partial_field_header() {
        let input = [138, 10, 85];
        let mut decoder = Decoder::new();

        let mut input_ref = &input[..1];
        assert_eq!(decoder.decode(&mut input_ref).unwrap(), None);

        input_ref = &input[1..];
        let header = decoder.decode_header(&mut input_ref).unwrap();
        assert_eq!(header.tag, 42);
        assert_eq!(header.wire_type, WireType::UInt64);
    }

    #[test]
    fn decode_out_of_order() {
        let input = [206, 10, 167, 138, 10, 85];
        let mut input_ref = &input[..];
        let mut decoder = Decoder::new();

        let header = decoder.decode_header(&mut input_ref).unwrap();
        assert_eq!(header.tag, 43);
        assert_eq!(header.wire_type, WireType::SInt64);

        let value = decoder.decode_sint64(&mut input_ref).unwrap();
        assert_eq!(value, -42);

        let error = decoder.decode(&mut input_ref).err().unwrap();
        assert_eq!(error, Error::Order { tag: 42 })
    }
}
