//! Veriform message decoder state machine

use super::{body, header, value};
use crate::{
    decoder::Event,
    error::Error,
    field::{Tag, WireType},
};

/// Decoder state machine
#[derive(Debug)]
pub(super) enum State {
    /// Reading the initial `vint64` header on a field
    Header(header::Decoder),

    /// Reading the `vint64` value of a field (either value itself or length prefix)
    Value(value::Decoder),

    /// Reading the body of a variable-length field
    Body(body::Decoder),
}

impl State {
    /// Process the given input data, advancing the slice for the amount of
    /// data processed, and returning the new state.
    pub(super) fn decode<'a>(
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
    pub(super) fn transition(event: &Event<'_>) -> Self {
        match event {
            Event::FieldHeader(header) => value::Decoder::new(header.wire_type).into(),
            Event::Bool(_) | Event::UInt64(_) | Event::SInt64(_) => State::default(),
            Event::LengthDelimiter { wire_type, length } => {
                if *length > 0 {
                    body::Decoder::new(*wire_type, *length).into()
                } else {
                    State::default()
                }
            }
            Event::SequenceHeader { length, .. } => {
                if *length > 0 {
                    body::Decoder::new(WireType::Sequence, *length).into()
                } else {
                    State::default()
                }
            }
            Event::ValueChunk {
                wire_type,
                remaining,
                ..
            } => {
                if *remaining > 0 {
                    body::Decoder::new(*wire_type, *remaining).into()
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

impl From<value::Decoder> for State {
    fn from(decoder: value::Decoder) -> State {
        State::Value(decoder)
    }
}
