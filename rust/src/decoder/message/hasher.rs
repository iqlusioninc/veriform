//! Verihash message hasher.
//!
//! WARNING: this is an experimental PoC-quality implementation!
//! It is NOT suitable for production use!
//!
//! # TODO
//!
//! - Nested message hashing
//! - Sequence hashing

// TODO(tarcieri): tests and test vectors!!!
// TODO(tarcieri): DRY out repeated logic in sequence hasher

use crate::{
    decoder::Event,
    error::Kind,
    field::{self, Tag, WireType},
    verihash,
};
use core::fmt::{self, Debug};
use digest::{generic_array::GenericArray, Digest};

/// Verihash message hasher.
///
/// This type computes a hash-based transcript of how a message was
/// decoded, driven by incoming decoding events.
pub struct Hasher<D: Digest> {
    /// Verihash hasher
    verihash: verihash::Hasher<D>,

    /// Current state of the decoder (or `None` if an error occurred)
    state: Option<State>,
}

impl<D> Hasher<D>
where
    D: Digest,
{
    /// Create a new [`Hasher`]
    pub fn new() -> Self {
        Self {
            verihash: verihash::Hasher::new(),
            state: Some(State::default()),
        }
    }

    /// Hash an incoming event
    pub fn hash_event(&mut self, event: &Event<'_>) -> Result<(), Kind> {
        if let Some(state) = self.state.take() {
            let new_state = state.transition(event, &mut self.verihash)?;
            self.state = Some(new_state);
            Ok(())
        } else {
            Err(Kind::Failed)
        }
    }

    /// Hash a digest of a nested message within this message
    pub fn hash_message_digest(
        &mut self,
        tag: Tag,
        digest: &GenericArray<u8, D::OutputSize>,
    ) -> Result<(), Kind> {
        match self.state {
            Some(State::Message { remaining }) if remaining == 0 => {
                self.verihash.tag(tag);
                self.verihash.fixed_size_value(WireType::Message, digest);
                self.state = Some(State::Initial);
                Ok(())
            }
            _ => Err(Kind::Hashing),
        }
    }

    /// Finish computing digest
    pub fn finish(self) -> Result<GenericArray<u8, D::OutputSize>, Kind> {
        if self.state == Some(State::Initial) {
            Ok(self.verihash.finish())
        } else {
            Err(Kind::Hashing)
        }
    }
}

impl<D> Default for Hasher<D>
where
    D: Digest,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<D> Debug for Hasher<D>
where
    D: Digest,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("message::Hasher").finish()
    }
}

#[derive(Debug, Eq, PartialEq)]
enum State {
    /// At the start of a message with no data processed
    Initial,

    /// Field header has been read
    Header(field::Header),

    /// Hashing a bytes field
    Bytes { remaining: usize },

    /// Hashing a string field
    String { remaining: usize },

    /// Hashing a message value
    Message { remaining: usize },

    /// Hashing a sequence value
    Sequence {
        wire_type: WireType,
        remaining: usize,
    },
}

impl Default for State {
    fn default() -> Self {
        State::Initial
    }
}

impl State {
    /// Transition to a new state based on an incoming event or return an error
    pub fn transition<D: Digest>(
        self,
        event: &Event<'_>,
        verihash: &mut verihash::Hasher<D>,
    ) -> Result<Self, Kind> {
        match event {
            Event::FieldHeader(header) => self.handle_field_header(header),
            Event::LengthDelimiter { wire_type, length } => {
                self.handle_length_delimiter(*wire_type, *length, verihash)
            }
            Event::Bool(_) | Event::UInt64(_) | Event::SInt64(_) => {
                self.handle_fixed_sized_value(event, verihash)
            }
            Event::ValueChunk {
                wire_type,
                bytes,
                remaining,
            } => self.handle_value_chunk(*wire_type, bytes, *remaining, verihash),
            Event::SequenceHeader { wire_type, length } => {
                self.handle_sequence_header(*wire_type, *length)
            }
        }
    }

    /// Handle an incoming field header
    fn handle_field_header(self, header: &field::Header) -> Result<Self, Kind> {
        if self == State::Initial {
            Ok(State::Header(*header))
        } else {
            Err(Kind::Hashing)
        }
    }

    /// Handle length delimiter event
    fn handle_length_delimiter<D: Digest>(
        self,
        wire_type: WireType,
        length: usize,
        verihash: &mut verihash::Hasher<D>,
    ) -> Result<Self, Kind> {
        if let State::Header(header) = self {
            if wire_type != header.wire_type {
                return Err(Kind::Hashing);
            }

            let new_state = match wire_type {
                WireType::Bytes => State::Bytes { remaining: length },
                WireType::String => State::String { remaining: length },
                WireType::Message => State::Message { remaining: length },
                _ => unreachable!(),
            };

            verihash.tag(header.tag);
            verihash.dynamically_sized_value(wire_type, length);

            Ok(new_state)
        } else {
            Err(Kind::Hashing)
        }
    }

    /// Handle hashing an incoming fixed-width value
    fn handle_fixed_sized_value<D: Digest>(
        self,
        value: &Event<'_>,
        verihash: &mut verihash::Hasher<D>,
    ) -> Result<Self, Kind> {
        if let State::Header(header) = self {
            match value {
                Event::Bool(value) => verihash.tagged_boolean(header.tag, *value),
                Event::UInt64(value) => verihash.tagged_uint64(header.tag, *value),
                Event::SInt64(value) => verihash.tagged_sint64(header.tag, *value),
                _ => unreachable!(),
            }
        } else {
            return Err(Kind::Hashing);
        }

        Ok(State::Initial)
    }

    /// Handle an incoming chunk of data in a value
    fn handle_value_chunk<D: Digest>(
        self,
        wire_type: WireType,
        bytes: &[u8],
        new_remaining: usize,
        verihash: &mut verihash::Hasher<D>,
    ) -> Result<Self, Kind> {
        // TODO(tarcieri): DRY this out
        let new_state = match self {
            State::Bytes { remaining } => {
                if wire_type != WireType::Bytes || remaining - bytes.len() != new_remaining {
                    return Err(Kind::Hashing);
                }

                if new_remaining == 0 {
                    State::Initial
                } else {
                    State::Bytes {
                        remaining: new_remaining,
                    }
                }
            }
            State::String { remaining } => {
                // TODO(tarcieri): use `unicode-normalization`?

                if wire_type != WireType::String || remaining - bytes.len() != new_remaining {
                    return Err(Kind::Hashing);
                }

                if new_remaining == 0 {
                    State::Initial
                } else {
                    State::String {
                        remaining: new_remaining,
                    }
                }
            }
            State::Message { remaining } => {
                if wire_type != WireType::Message || remaining - bytes.len() != new_remaining {
                    return Err(Kind::Hashing);
                }

                return Ok(State::Message {
                    remaining: new_remaining,
                });
            }
            State::Sequence {
                wire_type: value_type,
                remaining,
            } => {
                if wire_type != WireType::Sequence || remaining - bytes.len() != new_remaining {
                    return Err(Kind::Hashing);
                } else if new_remaining == 0 {
                    return Ok(State::Initial);
                } else {
                    return Ok(State::Sequence {
                        wire_type: value_type,
                        remaining: new_remaining,
                    });
                }
            }
            _ => {
                return Err(Kind::Hashing);
            }
        };

        verihash.input(bytes);
        Ok(new_state)
    }

    /// Handle an incoming sequence header
    fn handle_sequence_header(self, wire_type: WireType, length: usize) -> Result<Self, Kind> {
        if let State::Header(header) = self {
            if header.wire_type != WireType::Sequence {
                return Err(Kind::Hashing);
            }

            Ok(State::Sequence {
                wire_type,
                remaining: length,
            })
        } else {
            Err(Kind::Hashing)
        }
    }
}
