//! Sequence decoder

use super::{vint64, Event};
use crate::{error::Error, field::WireType};

/// Sequence decoder
pub struct Decoder {
    /// Wire type contained in this sequence
    wire_type: WireType,

    /// Remaining length in the sequence body
    remaining: usize,

    /// Current decoding state
    state: State,
}

impl Decoder {
    /// Create a new sequence decoder for the given wire type
    pub fn new(wire_type: WireType, length: usize) -> Self {
        Decoder {
            wire_type,
            remaining: length,
            state: State::Value(vint64::Decoder::new()),
        }
    }

    /// Decode a sequence from the given input
    pub fn decode<'a>(&mut self, input: &mut &'a [u8]) -> Result<Option<Event<'a>>, Error> {
        let event = match &mut self.state {
            State::Value(decoder) => {
                if let Some(value) = decoder.decode(input)? {
                    match self.wire_type {
                        WireType::UInt64 => Event::UInt64(value),
                        WireType::SInt64 => Event::SInt64(vint64::decode_zigzag(value)),
                        WireType::Sequence => Event::SequenceHeader {
                            wire_type: WireType::from_unmasked(value)?,
                            length: (value >> 3) as usize,
                        },
                        WireType::False | WireType::True => {
                            // TODO(tarcieri): support boolean sequences?
                            return Err(Error::Decode);
                        }
                        wire_type => {
                            debug_assert!(wire_type.is_length_delimited());
                            Event::LengthDelimiter {
                                wire_type,
                                length: value as usize,
                            }
                        }
                    }
                } else {
                    return Ok(None);
                }
            }
            State::Body {
                wire_type,
                remaining,
            } => {
                if input.is_empty() {
                    return Ok(None);
                }

                let chunk_size = if input.len() >= *remaining {
                    *remaining
                } else {
                    input.len()
                };

                let bytes = &input[..chunk_size];
                *input = &input[chunk_size..];

                let remaining = self.remaining.checked_sub(chunk_size).unwrap();

                Event::ValueChunk {
                    wire_type: *wire_type,
                    bytes,
                    remaining,
                }
            }
        };

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
            } => State::Body {
                wire_type: *wire_type,
                remaining: *remaining,
            },
            other => unreachable!("unexpected event: {:?}", other),
        };

        Ok(Some(event))
    }

    /// Decode an expected `message` field, returning an error for anything else
    pub fn decode_message<'a>(&mut self, input: &mut &'a [u8]) -> Result<&'a [u8], Error> {
        self.decode_value(input, WireType::Message)
    }

    /// Decode a length delimited value
    fn decode_value<'a>(
        &mut self,
        input: &mut &'a [u8],
        expected_type: WireType,
    ) -> Result<&'a [u8], Error> {
        if expected_type != self.wire_type {
            return Err(Error::WireType);
        }

        let length = self.decode_length_delimiter(input)?;

        if let Some(Event::ValueChunk {
            bytes, remaining, ..
        }) = self.decode(input)?
        {
            if remaining == 0 {
                debug_assert_eq!(length, bytes.len());
                return Ok(bytes);
            }
        }

        Err(Error::Decode)
    }

    /// Decode a length delimiter
    fn decode_length_delimiter(&mut self, input: &mut &[u8]) -> Result<usize, Error> {
        debug_assert!(self.wire_type.is_length_delimited());

        if let Some(Event::LengthDelimiter { length, .. }) = self.decode(input)? {
            Ok(length)
        } else {
            Err(Error::Decode)
        }
    }
}

/// Decoder state machine
#[derive(Debug)]
pub(super) enum State {
    /// Reading a `vint64` value (either value itself or length prefix)
    Value(vint64::Decoder),

    /// Reading the body of a variable-length value in a sequence
    Body {
        /// Wire type of the value body
        wire_type: WireType,

        /// Remaining data in the value
        remaining: usize,
    },
}
