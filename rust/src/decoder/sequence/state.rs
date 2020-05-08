//! Veriform sequence decoder state machine

use crate::{
    decoder::{
        vint64::{self, zigzag},
        Event,
    },
    error::Kind,
    field::WireType,
    message::Element,
};

/// Sequence decoder state machine
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

impl Default for State {
    fn default() -> State {
        State::Value(vint64::Decoder::new())
    }
}

impl State {
    /// Decode a sequence from the given input
    pub fn decode<'a>(
        &mut self,
        wire_type: WireType,
        input: &mut &'a [u8],
    ) -> Result<Option<Event<'a>>, Kind> {
        let event = match self {
            State::Value(decoder) => {
                if let Some(value) = decoder.decode(input)? {
                    match wire_type {
                        WireType::UInt64 => Event::UInt64(value),
                        WireType::SInt64 => Event::SInt64(zigzag::decode(value)),
                        WireType::Sequence => Event::SequenceHeader {
                            wire_type: WireType::from_unmasked(value),
                            length: (value >> 4) as usize,
                        },
                        WireType::False | WireType::True => {
                            // TODO(tarcieri): support boolean sequences?
                            return Err(Kind::Decode {
                                element: Element::Value,
                                wire_type,
                            });
                        }
                        wire_type => {
                            debug_assert!(
                                wire_type.is_dynamically_sized(),
                                "not a dynamically sized wire type: {:?}",
                                wire_type
                            );

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

                Event::ValueChunk {
                    wire_type: *wire_type,
                    bytes,
                    remaining: remaining.checked_sub(chunk_size).unwrap(),
                }
            }
        };

        Ok(Some(event))
    }
}
