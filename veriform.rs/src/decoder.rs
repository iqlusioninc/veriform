//! Veriform decoder

use crate::{
    field::{Header, WireType},
    Error,
};
use core::convert::TryFrom;

/// Veriform decoder: zero-copy pull parser which emits events based on
/// incoming data.
#[derive(Default)]
pub struct Decoder {
    /// Current state of the decoder (or `None` if an error occurred)
    state: Option<State>,
}

impl Decoder {
    /// Create a new decoder in an initial state
    pub fn new() -> Self {
        Self {
            state: Some(State::default()),
        }
    }

    /// Process the given input data, advancing the slice for the amount of
    /// data processed, and returning any decoded events.
    pub fn decode<'a>(&mut self, input: &mut &'a [u8]) -> Result<Option<Event<'a>>, Error> {
        if let Some(state) = self.state.take() {
            let (new_state, event) = state.decode(input)?;
            self.state = Some(new_state);
            Ok(event)
        } else {
            Err(Error::Failed)
        }
    }
}

/// Events emitted by Veriform's decoder
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Event<'a> {
    /// Consumed field header with the given tag and wire type
    FieldHeader(Header),

    /// Consumed an unsigned 64-bit integer
    UInt64(u64),

    /// Consumed a signed 64-bit integer
    SInt64(i64),

    /// Consumed the length of a nested message
    MessageLength(usize),

    /// Consumed the length of a field containing raw bytes
    BytesLength(usize),

    /// Consumed a portion of a nested message value in a field
    MessageChunk {
        /// Bytes in this chunk
        bytes: &'a [u8],

        /// Remaining bytes in the message
        remaining: usize,
    },

    /// Consumed a portion of binary data in a field
    BytesChunk {
        /// Bytes in this chunk
        bytes: &'a [u8],

        /// Remaining bytes in the message
        remaining: usize,
    },
}

/// Current decoder state
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
    pub fn decode<'a>(self, input: &mut &'a [u8]) -> Result<(Self, Option<Event<'a>>), Error> {
        match self {
            State::Header(header) => header.decode(input),
            State::Value(value) => value.decode(input),
            State::Body(body) => body.decode(input),
        }
    }

    /// Get the new state to transition to based on a given event
    fn transition(event: &Event<'_>) -> Self {
        match event {
            Event::FieldHeader(header) => ValueDecoder::new(header.wire_type).into(),
            Event::UInt64(_) | Event::SInt64(_) => State::default(),
            Event::MessageLength(length) => BodyDecoder::new(WireType::Message, *length).into(),
            Event::BytesLength(length) => BodyDecoder::new(WireType::Bytes, *length).into(),
            Event::MessageChunk { remaining, .. } => {
                if *remaining > 0 {
                    BodyDecoder::new(WireType::Message, *remaining).into()
                } else {
                    State::default()
                }
            }
            Event::BytesChunk { remaining, .. } => {
                if *remaining > 0 {
                    BodyDecoder::new(WireType::Bytes, *remaining).into()
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
#[derive(Default)]
struct HeaderDecoder(VInt64Decoder);

impl HeaderDecoder {
    /// Process the given input data, advancing the slice for the amount of
    /// data processed, and returning the new state.
    pub fn decode<'a>(mut self, input: &mut &'a [u8]) -> Result<(State, Option<Event<'a>>), Error> {
        if let Some(value) = self.0.decode(input)? {
            let event = Event::FieldHeader(Header::try_from(value)?);
            let new_state = State::transition(&event);
            Ok((new_state, Some(event)))
        } else {
            Ok((State::Header(self), None))
        }
    }
}

/// Decoder for field values
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
                WireType::UInt64 => Event::UInt64(value),
                WireType::SInt64 => Event::SInt64((value >> 1) as i64 ^ -((value & 1) as i64)),
                WireType::Message => Event::MessageLength(value as usize),
                WireType::Bytes => Event::BytesLength(value as usize),
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
struct BodyDecoder {
    /// Wire type we're decoding
    wire_type: WireType,

    /// Remaining bytes in this field body
    remaining: usize,
}

impl BodyDecoder {
    /// Create a new field value body decoder for the given wire type.
    ///
    /// Panics if the given wire type doesn't have a body
    pub fn new(wire_type: WireType, length: usize) -> Self {
        assert!(
            wire_type == WireType::Message || wire_type == WireType::Bytes,
            "can't create field body for {:?}",
            wire_type
        );

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

        let event = match self.wire_type {
            WireType::Message => Event::MessageChunk { bytes, remaining },
            WireType::Bytes => Event::BytesChunk { bytes, remaining },
            _ => unreachable!(), // Invariant maintained by `FieldBodyDecoder::new`
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
    use super::{Decoder, Event, WireType};

    macro_rules! try_decode {
        ($decoder:expr, $input:expr, $event:path) => {
            match $decoder.decode($input).unwrap() {
                Some($event(event)) => event,
                other => panic!(
                    concat!("expected ", stringify!($event), ", got: {:?}"),
                    other
                ),
            }
        };
    }

    #[test]
    fn decode_uint64() {
        let input = [66, 5, 85];
        let mut input_ref = &input[..];
        let mut decoder = Decoder::new();

        let header = try_decode!(decoder, &mut input_ref, Event::FieldHeader);
        assert_eq!(header.tag, 42);
        assert_eq!(header.wire_type, WireType::UInt64);

        let value = try_decode!(decoder, &mut input_ref, Event::UInt64);
        assert_eq!(value, 42);
    }

    #[test]
    fn decode_sint64() {
        let input = [102, 5, 167];
        let mut input_ref = &input[..];
        let mut decoder = Decoder::new();

        let header = try_decode!(decoder, &mut input_ref, Event::FieldHeader);
        assert_eq!(header.tag, 43);
        assert_eq!(header.wire_type, WireType::SInt64);

        let value = try_decode!(decoder, &mut input_ref, Event::SInt64);
        assert_eq!(value, -42);
    }

    #[test]
    fn decode_message() {
        let input = [21, 5, 33, 7];
        let mut input_ref = &input[..];
        let mut decoder = Decoder::new();

        let header = try_decode!(decoder, &mut input_ref, Event::FieldHeader);
        assert_eq!(header.tag, 1);
        assert_eq!(header.wire_type, WireType::Message);

        let msg_len = try_decode!(decoder, &mut input_ref, Event::MessageLength);
        assert_eq!(msg_len, 2);

        match decoder.decode(&mut input_ref).unwrap() {
            Some(Event::MessageChunk { bytes, remaining }) => {
                assert_eq!(remaining, 0);
                assert_eq!(bytes, &[33, 7]);
            }
            other => panic!(concat!("expected Event::MessageChunk, got: {:?}"), other),
        };
    }

    #[test]
    fn decode_bytes() {
        let input = [39, 11, 98, 121, 116, 101, 115];
        let mut input_ref = &input[..];
        let mut decoder = Decoder::new();

        let header = try_decode!(decoder, &mut input_ref, Event::FieldHeader);
        assert_eq!(header.tag, 2);
        assert_eq!(header.wire_type, WireType::Bytes);

        let msg_len = try_decode!(decoder, &mut input_ref, Event::BytesLength);
        assert_eq!(msg_len, 5);

        match decoder.decode(&mut input_ref).unwrap() {
            Some(Event::BytesChunk { bytes, remaining }) => {
                assert_eq!(remaining, 0);
                assert_eq!(bytes, &[98, 121, 116, 101, 115]);
            }
            other => panic!(concat!("expected Event::BytesChunk, got: {:?}"), other),
        };
    }

    #[test]
    fn decode_multiple() {
        let input = [66, 5, 85, 102, 5, 167];
        let mut input_ref = &input[..];
        let mut decoder = Decoder::new();

        let header = try_decode!(decoder, &mut input_ref, Event::FieldHeader);
        assert_eq!(header.tag, 42);
        assert_eq!(header.wire_type, WireType::UInt64);

        let value = try_decode!(decoder, &mut input_ref, Event::UInt64);
        assert_eq!(value, 42);

        let header = try_decode!(decoder, &mut input_ref, Event::FieldHeader);
        assert_eq!(header.tag, 43);
        assert_eq!(header.wire_type, WireType::SInt64);

        let value = try_decode!(decoder, &mut input_ref, Event::SInt64);
        assert_eq!(value, -42);
    }

    #[test]
    fn decode_partial_field_header() {
        let input = [66, 5, 85];
        let mut decoder = Decoder::new();

        let mut input_ref = &input[..1];
        assert_eq!(decoder.decode(&mut input_ref).unwrap(), None);

        input_ref = &input[1..];
        let header = try_decode!(decoder, &mut input_ref, Event::FieldHeader);
        assert_eq!(header.tag, 42);
        assert_eq!(header.wire_type, WireType::UInt64);
    }
}
