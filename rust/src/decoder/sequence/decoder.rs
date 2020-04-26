//! Veriform sequence decoder

use super::state::State;
use crate::{
    decoder::{vint64, Decodable, Event},
    error::Error,
    field::WireType,
    message::Element,
};

/// Sequence decoder
pub struct Decoder {
    /// Wire type contained in this sequence
    wire_type: WireType,

    /// Total length of the sequence
    length: usize,

    /// Remaining length in the sequence body
    remaining: usize,

    /// Current decoding state
    state: State,
}

impl Decodable for Decoder {
    fn decode<'a>(&mut self, input: &mut &'a [u8]) -> Result<Option<Event<'a>>, Error> {
        let orig_input_len = input.len();
        let maybe_event = self.state.decode(self.wire_type, input)?;
        let consumed = orig_input_len.checked_sub(input.len()).unwrap();
        self.remaining = self.remaining.checked_sub(consumed).unwrap();

        if let Some(event) = &maybe_event {
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
            return Err(Error::UnexpectedWireType {
                actual: self.wire_type,
                wanted: expected_type,
            });
        }

        debug_assert!(
            self.wire_type.is_dynamically_sized(),
            "not a dynamically sized wire type: {:?}",
            self.wire_type
        );

        let length = match self.decode(input)? {
            Some(Event::LengthDelimiter { length, .. }) => Ok(length),
            _ => Err(Error::Decode {
                element: Element::LengthDelimiter,
                wire_type: self.wire_type,
            }),
        }?;

        match self.decode(input)? {
            Some(Event::ValueChunk {
                bytes, remaining, ..
            }) => {
                if remaining == 0 {
                    debug_assert_eq!(length, bytes.len());
                    Ok(bytes)
                } else {
                    Err(Error::Truncated {
                        remaining,
                        wire_type: self.wire_type,
                    })
                }
            }
            _ => Err(Error::Decode {
                element: Element::Value,
                wire_type: self.wire_type,
            }),
        }
    }
}

impl Decoder {
    /// Create a new sequence decoder for the given wire type
    pub fn new(wire_type: WireType, length: usize) -> Self {
        Decoder {
            wire_type,
            length,
            remaining: length,
            state: State::default(),
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
}

#[cfg(test)]
mod tests {
    use super::{Decodable, Decoder, WireType};

    #[test]
    fn decode_uint64_sequence() {
        let input = [3, 5, 7];
        let mut input_ref = &input[..];
        let mut decoder = Decoder::new(WireType::UInt64, input.len());

        assert_eq!(1, decoder.decode_uint64(&mut input_ref).unwrap());
        assert_eq!(2, decoder.decode_uint64(&mut input_ref).unwrap());
        assert_eq!(3, decoder.decode_uint64(&mut input_ref).unwrap());
        assert!(input_ref.is_empty());
    }

    #[test]
    fn decode_sint64_sequence() {
        let input = [3, 7, 11];
        let mut input_ref = &input[..];
        let mut decoder = Decoder::new(WireType::SInt64, input.len());

        for n in &[-1, -2, -3] {
            assert_eq!(*n, decoder.decode_sint64(&mut input_ref).unwrap());
        }

        assert!(input_ref.is_empty());
    }

    #[test]
    fn decode_bytes_sequence() {
        let input = [7, 102, 111, 111, 7, 98, 97, 114, 7, 98, 97, 122];
        let mut input_ref = &input[..];

        let mut decoder = Decoder::new(WireType::Bytes, input.len());

        for &b in &[b"foo", b"bar", b"baz"] {
            assert_eq!(b, decoder.decode_bytes(&mut input_ref).unwrap());
        }

        assert!(input_ref.is_empty());
    }

    #[test]
    fn decode_string_sequence() {
        let input = [7, 102, 111, 111, 7, 98, 97, 114, 7, 98, 97, 122];
        let mut input_ref = &input[..];

        let mut decoder = Decoder::new(WireType::String, input.len());

        for &s in &["foo", "bar", "baz"] {
            assert_eq!(s, decoder.decode_string(&mut input_ref).unwrap());
        }

        assert!(input_ref.is_empty());
    }
}
