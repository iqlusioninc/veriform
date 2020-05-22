//! Veriform sequence decoder

use super::{hasher::Hasher, state::State};
use crate::{
    decoder::{vint64, Decodable, Event},
    error::{self, Error},
    field::WireType,
    message::Element,
    verihash::DigestOutput,
};
use digest::Digest;

/// Sequence decoder
pub(crate) struct Decoder<D: Digest> {
    /// Wire type contained in this sequence
    wire_type: WireType,

    /// Total length of the sequence
    length: usize,

    /// Remaining length in the sequence body
    remaining: usize,

    /// Current decoding state
    state: State,

    /// Verihash message hasher
    hasher: Option<Hasher<D>>,
}

impl<D> Decoder<D>
where
    D: Digest,
{
    /// Create a new sequence decoder for the given wire type
    pub fn new(wire_type: WireType, length: usize) -> Self {
        Self {
            wire_type,
            length,
            remaining: length,
            state: State::default(),
            hasher: Some(Hasher::new(wire_type)), // TODO(tarcieri): support for disabling hasher
        }
    }

    /// Get the current position (i.e. number of bytes processed) in the
    /// sequence being decoded
    pub fn position(&self) -> usize {
        self.length.checked_sub(self.remaining).unwrap()
    }

    /// Get the number of bytes remaining in the sequence
    pub fn remaining(&self) -> usize {
        self.remaining
    }

    /// Perform a state transition after receiving an event
    fn transition<'a>(&mut self, event: &Event<'a>) {
        self.state = match &event {
            Event::LengthDelimiter { wire_type, length }
            | Event::SequenceHeader { wire_type, length } => State::Body {
                wire_type: *wire_type,
                remaining: *length,
            },
            Event::UInt64(_) | Event::SInt64(_) => State::Value(vint64::Decoder::new()),
            Event::ValueChunk {
                wire_type,
                remaining,
                ..
            } => {
                if *remaining > 0 {
                    State::Body {
                        wire_type: *wire_type,
                        remaining: *remaining,
                    }
                } else {
                    State::default()
                }
            }
            other => unreachable!("unexpected event: {:?}", other),
        };
    }

    /// Hash a digest of a nested message within this sequence
    pub fn hash_message_digest(&mut self, digest: &DigestOutput<D>) -> Result<(), Error> {
        if let Some(hasher) = &mut self.hasher {
            hasher.hash_message_digest(digest)?;
        }

        Ok(())
    }

    /// Compute a Verihash digest of the sequence we're decoding.
    pub fn compute_digest(self) -> Result<Option<DigestOutput<D>>, Error> {
        self.hasher.map(|hasher| hasher.finish()).transpose()
    }
}

impl<D> Decodable for Decoder<D>
where
    D: Digest,
{
    fn decode<'a>(&mut self, input: &mut &'a [u8]) -> Result<Option<Event<'a>>, Error> {
        let orig_input_len = input.len();
        let maybe_event = self.state.decode(self.wire_type, input)?;
        let consumed = orig_input_len.checked_sub(input.len()).unwrap();
        self.remaining = self.remaining.checked_sub(consumed).unwrap();

        if let Some(event) = &maybe_event {
            if let Some(hasher) = &mut self.hasher {
                hasher.hash_event(event)?;
            }

            self.transition(&event);
        }

        Ok(maybe_event)
    }

    fn decode_dynamically_sized_value<'a>(
        &mut self,
        expected_type: WireType,
        input: &mut &'a [u8],
    ) -> Result<&'a [u8], Error> {
        if expected_type != self.wire_type {
            return Err(error::Kind::UnexpectedWireType {
                actual: self.wire_type,
                wanted: expected_type,
            }
            .into());
        }

        debug_assert!(
            self.wire_type.is_dynamically_sized(),
            "not a dynamically sized wire type: {:?}",
            self.wire_type
        );

        let length = match self.decode(input)? {
            Some(Event::LengthDelimiter { length, .. }) => Ok(length),
            _ => Err(error::Kind::Decode {
                element: Element::LengthDelimiter,
                wire_type: self.wire_type,
            }
            .position(self.length.checked_sub(self.remaining).unwrap())),
        }?;

        match self.decode(input)? {
            Some(Event::ValueChunk {
                bytes, remaining, ..
            }) => {
                if remaining == 0 {
                    debug_assert_eq!(length, bytes.len());
                    Ok(bytes)
                } else {
                    Err(error::Kind::Truncated {
                        remaining,
                        wire_type: self.wire_type,
                    }
                    .into())
                }
            }
            _ => Err(error::Kind::Decode {
                element: Element::Value,
                wire_type: self.wire_type,
            }
            .into()),
        }
    }
}

#[cfg(all(test, features = "sha2"))]
mod tests {
    use super::{Decodable, Decoder, WireType};
    use sha2::Sha256;

    #[test]
    fn decode_uint64_sequence() {
        let input = [3, 5, 7];
        let mut input_ref = &input[..];
        let mut decoder: Decoder<Sha256> = Decoder::new(WireType::UInt64, input.len());

        assert_eq!(1, decoder.decode_uint64(&mut input_ref).unwrap());
        assert_eq!(2, decoder.decode_uint64(&mut input_ref).unwrap());
        assert_eq!(3, decoder.decode_uint64(&mut input_ref).unwrap());
        assert!(input_ref.is_empty());
    }

    #[test]
    fn decode_sint64_sequence() {
        let input = [3, 7, 11];
        let mut input_ref = &input[..];
        let mut decoder: Decoder<Sha256> = Decoder::new(WireType::SInt64, input.len());

        for n in &[-1, -2, -3] {
            assert_eq!(*n, decoder.decode_sint64(&mut input_ref).unwrap());
        }

        assert!(input_ref.is_empty());
    }

    #[test]
    fn decode_bytes_sequence() {
        let input = [7, 102, 111, 111, 7, 98, 97, 114, 7, 98, 97, 122];
        let mut input_ref = &input[..];
        let mut decoder: Decoder<Sha256> = Decoder::new(WireType::Bytes, input.len());

        for &b in &[b"foo", b"bar", b"baz"] {
            assert_eq!(b, decoder.decode_bytes(&mut input_ref).unwrap());
        }

        assert!(input_ref.is_empty());
    }

    #[test]
    fn decode_string_sequence() {
        let input = [7, 102, 111, 111, 7, 98, 97, 114, 7, 98, 97, 122];
        let mut input_ref = &input[..];
        let mut decoder: Decoder<Sha256> = Decoder::new(WireType::String, input.len());

        for &s in &["foo", "bar", "baz"] {
            assert_eq!(s, decoder.decode_string(&mut input_ref).unwrap());
        }

        assert!(input_ref.is_empty());
    }
}
