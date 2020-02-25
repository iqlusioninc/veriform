/// Decoder for the bodies of variable-length field values
use super::{Event, State};
use crate::{error::Error, field::WireType};

/// Decoder for the bodies of variable-length field values
#[derive(Debug)]
pub(super) struct Decoder {
    /// Wire type we're decoding
    wire_type: WireType,

    /// Remaining bytes in this field body
    remaining: usize,
}

impl Decoder {
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

impl From<Decoder> for State {
    fn from(decoder: Decoder) -> State {
        State::Body(decoder)
    }
}
