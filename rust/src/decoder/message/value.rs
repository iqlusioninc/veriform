//! Decoder for field values

use super::State;
use crate::{
    decoder::{vint64, Event},
    error::Error,
    field::WireType,
};

/// Decoder for field values
#[derive(Debug)]
pub(super) struct Decoder {
    /// Create a new decoder for the `vint64` length prefix or value
    decoder: vint64::Decoder,

    /// Wire type we're decoding
    wire_type: WireType,
}

impl Decoder {
    /// Create a new value decoder for the given wire type
    pub fn new(wire_type: WireType) -> Self {
        Self {
            decoder: vint64::Decoder::new(),
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
                WireType::SInt64 => Event::SInt64(vint64::decode_zigzag(value)),
                WireType::Sequence => Event::SequenceHeader {
                    wire_type: WireType::from_unmasked(value)?,
                    length: (value >> 4) as usize,
                },
                wire_type => {
                    debug_assert!(
                        wire_type.is_length_delimited(),
                        "not a length-delimited wire type: {:?}",
                        wire_type
                    );

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
