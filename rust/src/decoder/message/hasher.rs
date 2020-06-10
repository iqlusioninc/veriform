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
    error::{self, Error},
    field::{self, Tag, WireType},
    verihash::{self, DigestOutput},
};
use core::fmt::{self, Debug};
use digest::Digest;

/// Verihash message hasher.
///
/// This type computes a hash-based transcript of how a message was
/// decoded, driven by incoming decoding events.
pub(super) struct Hasher<D: Digest> {
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
    pub fn hash_event(&mut self, event: &Event<'_>) -> Result<(), Error> {
        if let Some(state) = self.state.take() {
            let new_state = state.transition(event, &mut self.verihash)?;
            self.state = Some(new_state);
            Ok(())
        } else {
            Err(error::Kind::Failed.into())
        }
    }

    /// Hash a digest of a nested message within this message
    pub fn hash_message_digest(&mut self, tag: Tag, digest: &DigestOutput<D>) -> Result<(), Error> {
        match self.state {
            Some(State::Message { remaining }) if remaining == 0 => {
                self.verihash.tag(tag);
                self.verihash.fixed_size_value(WireType::Message, digest);
                self.state = Some(State::Initial);
                Ok(())
            }
            _ => Err(error::Kind::Hashing.into()),
        }
    }

    /// Hash a digest of a sequence within this message
    pub fn hash_sequence_digest(
        &mut self,
        tag: Tag,
        digest: &DigestOutput<D>,
    ) -> Result<(), Error> {
        match self.state {
            Some(State::Sequence { remaining, .. }) if remaining == 0 => {
                self.verihash.tag(tag);
                self.verihash.fixed_size_value(WireType::Sequence, digest);
                self.state = Some(State::Initial);
                Ok(())
            }
            _ => Err(error::Kind::Hashing.into()),
        }
    }

    /// Finish computing digest
    pub fn finish(self) -> Result<DigestOutput<D>, Error> {
        if self.state == Some(State::Initial) {
            Ok(self.verihash.finalize())
        } else {
            Err(error::Kind::Hashing.into())
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
    ) -> Result<Self, Error> {
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
    fn handle_field_header(self, header: &field::Header) -> Result<Self, Error> {
        if self == State::Initial {
            Ok(State::Header(*header))
        } else {
            Err(error::Kind::Hashing.into())
        }
    }

    /// Handle length delimiter event
    fn handle_length_delimiter<D: Digest>(
        self,
        wire_type: WireType,
        length: usize,
        verihash: &mut verihash::Hasher<D>,
    ) -> Result<Self, Error> {
        if let State::Header(header) = self {
            if wire_type != header.wire_type {
                return Err(error::Kind::Hashing.into());
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
            Err(error::Kind::Hashing.into())
        }
    }

    /// Handle hashing an incoming fixed-width value
    fn handle_fixed_sized_value<D: Digest>(
        self,
        value: &Event<'_>,
        verihash: &mut verihash::Hasher<D>,
    ) -> Result<Self, Error> {
        if let State::Header(header) = self {
            match value {
                Event::Bool(value) => verihash.tagged_boolean(header.tag, *value),
                Event::UInt64(value) => verihash.tagged_uint64(header.tag, *value),
                Event::SInt64(value) => verihash.tagged_sint64(header.tag, *value),
                _ => unreachable!(),
            }
        } else {
            return Err(error::Kind::Hashing.into());
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
    ) -> Result<Self, Error> {
        // TODO(tarcieri): DRY this out
        let new_state = match self {
            State::Bytes { remaining } => {
                if wire_type != WireType::Bytes || remaining - bytes.len() != new_remaining {
                    return Err(error::Kind::Hashing.into());
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
                if wire_type != WireType::String || remaining - bytes.len() != new_remaining {
                    return Err(error::Kind::Hashing.into());
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
                    return Err(error::Kind::Hashing.into());
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
                    return Err(error::Kind::Hashing.into());
                } else {
                    return Ok(State::Sequence {
                        wire_type: value_type,
                        remaining: new_remaining,
                    });
                }
            }
            _ => {
                return Err(error::Kind::Hashing.into());
            }
        };

        verihash.update(bytes);
        Ok(new_state)
    }

    /// Handle an incoming sequence header
    fn handle_sequence_header(self, wire_type: WireType, length: usize) -> Result<Self, Error> {
        if let State::Header(header) = self {
            if header.wire_type != WireType::Sequence {
                return Err(error::Kind::Hashing.into());
            }

            Ok(State::Sequence {
                wire_type,
                remaining: length,
            })
        } else {
            Err(error::Kind::Hashing.into())
        }
    }
}
