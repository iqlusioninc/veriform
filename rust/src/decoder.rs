//! Veriform decoder

use crate::{
    field::{Header, Tag, WireType},
    Error,
};
use core::{convert::TryFrom, str};

/// Veriform decoder: zero-copy pull parser which emits events based on
/// incoming data.
#[derive(Debug)]
pub struct Decoder {
    /// Current state of the decoder (or `None` if an error occurred)
    state: Option<State>,

    /// Last field tag that was decoded (to ensure monotonicity)
    last_tag: Option<Tag>,
}

impl Default for Decoder {
    fn default() -> Self {
        Self {
            state: Some(State::default()),
            last_tag: None,
        }
    }
}

impl Decoder {
    /// Create a new decoder in an initial state
    pub fn new() -> Self {
        Self::default()
    }

    /// Process the given input data, advancing the slice for the amount of
    /// data processed, and returning any decoded events.
    pub fn decode<'a>(&mut self, input: &mut &'a [u8]) -> Result<Option<Event<'a>>, Error> {
        if let Some(state) = self.state.take() {
            let (new_state, event) = state.decode(input, self.last_tag)?;

            if let Some(Event::FieldHeader(header)) = &event {
                self.last_tag = Some(header.tag);
            }

            self.state = Some(new_state);
            Ok(event)
        } else {
            Err(Error::Failed)
        }
    }

    /// Decode an expected field header, returning an error for anything else
    pub fn decode_header(&mut self, input: &mut &[u8]) -> Result<Header, Error> {
        if let Some(Event::FieldHeader(header)) = self.decode(input)? {
            Ok(header)
        } else {
            Err(Error::Decode)
        }
    }

    /// Decode an expected `uint64`, returning an error for anything else
    pub fn decode_uint64(&mut self, input: &mut &[u8]) -> Result<u64, Error> {
        if let Some(Event::UInt64(value)) = self.decode(input)? {
            Ok(value)
        } else {
            Err(Error::Decode)
        }
    }

    /// Decode an expected `sint64`, returning an error for anything else
    pub fn decode_sint64(&mut self, input: &mut &[u8]) -> Result<i64, Error> {
        match self.decode(input)? {
            Some(Event::SInt64(value)) => Ok(value),
            _ => Err(Error::Decode),
        }
    }

    /// Decode an expected `message` field, returning an error for anything else
    pub fn decode_message<'a>(&mut self, input: &mut &'a [u8]) -> Result<&'a [u8], Error> {
        self.decode_value(input, WireType::Message)
    }

    /// Decode an expected `bytes` field, returning an error for anything else
    pub fn decode_bytes<'a>(&mut self, input: &mut &'a [u8]) -> Result<&'a [u8], Error> {
        self.decode_value(input, WireType::Bytes)
    }

    /// Decode an expected `string` field, returning an error for anything else
    pub fn decode_string<'a>(&mut self, input: &mut &'a [u8]) -> Result<&'a str, Error> {
        let bytes = self.decode_value(input, WireType::String)?;
        str::from_utf8(bytes).map_err(|e| Error::Utf8 {
            valid_up_to: e.valid_up_to(),
        })
    }

    /// Decode a length delimited value, expecting the given wire type
    fn decode_value<'a>(
        &mut self,
        input: &mut &'a [u8],
        expected_type: WireType,
    ) -> Result<&'a [u8], Error> {
        let length = self.decode_length_delimiter(input, expected_type)?;

        if let Some(Event::ValueChunk {
            wire_type,
            bytes,
            remaining,
        }) = self.decode(input)?
        {
            if wire_type == expected_type && remaining == 0 {
                debug_assert_eq!(length, bytes.len());
                return Ok(bytes);
            }
        }

        Err(Error::Decode)
    }

    /// Decode the length delimiter, expecting the given wire type
    fn decode_length_delimiter(
        &mut self,
        input: &mut &[u8],
        expected_type: WireType,
    ) -> Result<usize, Error> {
        debug_assert!(expected_type.is_length_delimited());

        if let Some(Event::LengthDelimiter { wire_type, length }) = self.decode(input)? {
            if wire_type == expected_type {
                return Ok(length);
            }
        }

        Err(Error::Decode)
    }
}

/// Events emitted by Veriform's decoder
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Event<'a> {
    /// Consumed field header with the given tag and wire type
    FieldHeader(Header),

    /// Consumed a boolean value
    Bool(bool),

    /// Consumed an unsigned 64-bit integer
    UInt64(u64),

    /// Consumed a signed 64-bit integer
    SInt64(i64),

    /// Consumed a length delimiter for the given wire type
    LengthDelimiter {
        /// Wire type of the value this length delimits
        wire_type: WireType,

        /// Length of the field body (sans delimiter)
        length: usize,
    },

    /// Consumed a chunk of a length-delimited value
    ValueChunk {
        /// Wire type of the value being consumed
        wire_type: WireType,

        /// Bytes in this chunk
        bytes: &'a [u8],

        /// Remaining bytes in the message
        remaining: usize,
    },
}

/// Current decoder state
#[derive(Debug)]
enum State {
    /// Reading the initial `vint64` header on a field
    Header(HeaderDecoder),

    /// Reading the `vint64` value of a field (either value itself or length prefix)
    Value(ValueDecoder),

    /// Reading the body of a variable-length field
    Body(BodyDecoder),
}

impl State {
    /// Process the given input data, advancing the slice for the amount of
    /// data processed, and returning the new state.
    pub fn decode<'a>(
        self,
        input: &mut &'a [u8],
        last_tag: Option<Tag>,
    ) -> Result<(Self, Option<Event<'a>>), Error> {
        match self {
            State::Header(header) => header.decode(input, last_tag),
            State::Value(value) => value.decode(input),
            State::Body(body) => body.decode(input),
        }
    }

    /// Get the new state to transition to based on a given event
    fn transition(event: &Event<'_>) -> Self {
        match event {
            Event::FieldHeader(header) => ValueDecoder::new(header.wire_type).into(),
            Event::LengthDelimiter { wire_type, length } => {
                BodyDecoder::new(*wire_type, *length).into()
            }
            Event::Bool(_) | Event::UInt64(_) | Event::SInt64(_) => State::default(),
            Event::ValueChunk {
                wire_type,
                remaining,
                ..
            } => {
                if *remaining > 0 {
                    BodyDecoder::new(*wire_type, *remaining).into()
                } else {
                    State::default()
                }
            }
        }
    }
}

impl Default for State {
    fn default() -> State {
        State::Header(Default::default())
    }
}

/// Decoder for field headers
#[derive(Default, Debug)]
struct HeaderDecoder(VInt64Decoder);

impl HeaderDecoder {
    /// Process the given input data, advancing the slice for the amount of
    /// data processed, and returning the new state.
    pub fn decode<'a>(
        mut self,
        input: &mut &'a [u8],
        last_tag: Option<Tag>,
    ) -> Result<(State, Option<Event<'a>>), Error> {
        if let Some(value) = self.0.decode(input)? {
            let header = Header::try_from(value)?;

            // Ensure field ordering is monotonically increasing
            if let Some(tag) = last_tag {
                if header.tag < tag {
                    return Err(Error::Order { tag: header.tag });
                }
            }

            let event = Event::FieldHeader(header);
            let new_state = State::transition(&event);
            Ok((new_state, Some(event)))
        } else {
            Ok((State::Header(self), None))
        }
    }
}

/// Decoder for field values
#[derive(Debug)]
struct ValueDecoder {
    /// Create a new decoder for the `vint64` length prefix or value
    decoder: VInt64Decoder,

    /// Wire type we're decoding
    wire_type: WireType,
}

impl ValueDecoder {
    /// Create a new value decoder for the given wire type
    pub fn new(wire_type: WireType) -> Self {
        Self {
            decoder: VInt64Decoder::new(),
            wire_type,
        }
    }

    /// Process the given input data, advancing the slice for the amount of
    /// data processed, and returning the new state.
    pub fn decode<'a>(mut self, input: &mut &'a [u8]) -> Result<(State, Option<Event<'a>>), Error> {
        if let Some(value) = self.decoder.decode(input)? {
            let event = match self.wire_type {
                WireType::False => Event::Bool(false),
                WireType::True => Event::Bool(true),
                WireType::UInt64 => Event::UInt64(value),
                WireType::SInt64 => Event::SInt64((value >> 1) as i64 ^ -((value & 1) as i64)),
                wire_type => {
                    debug_assert!(wire_type.is_length_delimited());
                    Event::LengthDelimiter {
                        wire_type,
                        length: value as usize,
                    }
                }
            };

            let new_state = State::transition(&event);
            Ok((new_state, Some(event)))
        } else {
            Ok((State::Value(self), None))
        }
    }
}

impl From<ValueDecoder> for State {
    fn from(decoder: ValueDecoder) -> State {
        State::Value(decoder)
    }
}

/// Decoder for the bodies of variable-length field values
#[derive(Debug)]
struct BodyDecoder {
    /// Wire type we're decoding
    wire_type: WireType,

    /// Remaining bytes in this field body
    remaining: usize,
}

impl BodyDecoder {
    /// Create a new field value body decoder for the given wire type.
    ///
    /// Panics if the given wire type isn't a length-delimited type (debug-only).
    pub fn new(wire_type: WireType, length: usize) -> Self {
        debug_assert!(wire_type.is_length_delimited());

        Self {
            wire_type,
            remaining: length,
        }
    }

    /// Process the given input data, advancing the slice for the amount of
    /// data processed, and returning the new state.
    pub fn decode<'a>(self, input: &mut &'a [u8]) -> Result<(State, Option<Event<'a>>), Error> {
        if input.is_empty() {
            return Ok((self.into(), None));
        }

        let chunk_size = if input.len() >= self.remaining {
            self.remaining
        } else {
            input.len()
        };

        let bytes = &input[..chunk_size];
        *input = &input[chunk_size..];

        let remaining = self.remaining.checked_sub(chunk_size).unwrap();
        let event = Event::ValueChunk {
            wire_type: self.wire_type,
            bytes,
            remaining,
        };
        let new_state = State::transition(&event);

        Ok((new_state, Some(event)))
    }
}

impl From<BodyDecoder> for State {
    fn from(decoder: BodyDecoder) -> State {
        State::Body(decoder)
    }
}

/// Decoder for `vint64` values
#[derive(Clone, Debug, Default)]
struct VInt64Decoder {
    /// Length of the field header `vint64` (if known)
    length: Option<usize>,

    /// Position we are at reading in the input buffer
    pos: usize,

    /// Incoming data buffer
    buffer: [u8; 9],
}

impl VInt64Decoder {
    /// Create a new [`VInt64Decoder`]
    pub fn new() -> Self {
        Self::default()
    }

    /// Decode a `vint64` from the incoming data
    pub fn decode(&mut self, input: &mut &[u8]) -> Result<Option<u64>, Error> {
        if let Some(length) = self.length {
            self.fill_buffer(length, input);
            return self.maybe_decode(length);
        }

        if let Some(&hint) = input.first() {
            self.length = Some(vint64::length_hint(hint));
            self.decode(input)
        } else {
            Ok(None)
        }
    }

    /// Fill the internal buffer with data, returning a [`FieldHeader`] if we're complete
    fn fill_buffer(&mut self, length: usize, input: &mut &[u8]) {
        let remaining = length.checked_sub(self.pos).unwrap();

        if input.len() < remaining {
            let new_pos = self.pos.checked_add(input.len()).unwrap();
            self.buffer[self.pos..new_pos].copy_from_slice(*input);
            self.pos = new_pos;
            *input = &[];
        } else {
            self.buffer[self.pos..length].copy_from_slice(&input[..remaining]);
            self.pos += remaining;
            *input = &input[remaining..];
        }
    }

    /// Attempt to decode the internal buffer if we've read its full contents
    fn maybe_decode(&self, length: usize) -> Result<Option<u64>, Error> {
        if self.pos < length {
            return Ok(None);
        }

        let mut buffer = &self.buffer[..length];
        vint64::decode(&mut buffer)
            .map(Some)
            .map_err(|_| Error::Decode)
    }
}

#[cfg(test)]
mod tests {
    use super::{Decoder, WireType};
    use crate::error::Error;

    #[test]
    fn decode_false() {
        let input = [66, 5];
        let mut input_ref = &input[..];
        let mut decoder = Decoder::new();

        let header = decoder.decode_header(&mut input_ref).unwrap();
        assert_eq!(header.tag, 42);
        assert_eq!(header.wire_type, WireType::False);
    }

    #[test]
    fn decode_true() {
        let input = [102, 5];
        let mut input_ref = &input[..];
        let mut decoder = Decoder::new();

        let header = decoder.decode_header(&mut input_ref).unwrap();
        assert_eq!(header.tag, 43);
        assert_eq!(header.wire_type, WireType::True);
    }

    #[test]
    fn decode_uint64() {
        let input = [74, 5, 85];
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
        let input = [110, 5, 167];
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
    fn decode_message() {
        let input = [25, 5, 69, 7];
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
    fn decode_bytes() {
        let input = [43, 11, 98, 121, 116, 101, 115];
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
        let input = [77, 7, 98, 97, 122];
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
    fn decode_multiple() {
        let input = [74, 5, 85, 110, 5, 167];
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
        let input = [74, 5, 85];
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
        let input = [110, 5, 167, 74, 5, 85];
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
