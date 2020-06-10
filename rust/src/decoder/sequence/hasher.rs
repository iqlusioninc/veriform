//! Verihash sequence hasher.
//!
//! WARNING: this is an experimental PoC-quality implementation!
//! It is NOT suitable for production use!

// TODO(tarcieri): tests and test vectors!!!
// TODO(tarcieri): DRY out repeated message/sequence code into `verihash::Hasher`

use crate::{
    decoder::Event,
    error::{self, Error},
    field::WireType,
    verihash::{self, DigestOutput},
};
use core::fmt::{self, Debug};
use digest::Digest;

/// Verihash sequence hasher.
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
    pub fn new(wire_type: WireType) -> Self {
        let mut verihash = verihash::Hasher::new();

        // Domain separate sequence hashes by their contained wire type
        verihash.update(&[wire_type.to_u8()]);

        Self {
            verihash,
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
    pub fn hash_message_digest(&mut self, digest: &DigestOutput<D>) -> Result<(), Error> {
        match self.state {
            Some(State::Message { remaining }) if remaining == 0 => {
                self.verihash.update(digest);
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

impl<D> Debug for Hasher<D>
where
    D: Digest,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("sequence::Hasher").finish()
    }
}

#[derive(Debug, Eq, PartialEq)]
enum State {
    /// At the start of a message with no data processed
    Initial,

    /// Hashing a bytes field
    Bytes { remaining: usize },

    /// Hashing a string field
    String { remaining: usize },

    /// Hashing a message value
    Message { remaining: usize },
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
            Event::LengthDelimiter { wire_type, length } => {
                self.handle_length_delimiter(*wire_type, *length, verihash)
            }
            Event::UInt64(_) | Event::SInt64(_) => self.handle_fixed_sized_value(event, verihash),
            Event::ValueChunk {
                wire_type,
                bytes,
                remaining,
            } => self.handle_value_chunk(*wire_type, bytes, *remaining, verihash),
            _ => Err(error::Kind::Hashing.into()),
        }
    }

    /// Handle length delimiter event
    fn handle_length_delimiter<D: Digest>(
        self,
        wire_type: WireType,
        length: usize,
        verihash: &mut verihash::Hasher<D>,
    ) -> Result<Self, Error> {
        if self != State::Initial {
            return Err(error::Kind::Hashing.into());
        }

        let new_state = match wire_type {
            WireType::Bytes => State::Bytes { remaining: length },
            WireType::String => State::String { remaining: length },
            WireType::Message => State::Message { remaining: length },
            _ => unreachable!(),
        };

        verihash.dynamically_sized_value(wire_type, length);
        Ok(new_state)
    }

    /// Handle hashing an incoming fixed-width value
    fn handle_fixed_sized_value<D: Digest>(
        self,
        value: &Event<'_>,
        verihash: &mut verihash::Hasher<D>,
    ) -> Result<Self, Error> {
        if self != State::Initial {
            return Err(error::Kind::Hashing.into());
        }

        match value {
            Event::UInt64(value) => {
                verihash.fixed_size_value(WireType::UInt64, &value.to_le_bytes())
            }
            Event::SInt64(value) => {
                verihash.fixed_size_value(WireType::SInt64, &value.to_le_bytes())
            }
            _ => unreachable!(),
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
        // TODO(tarcieri): DRY this out (especially with the message decoder)
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
                // We don't actually handle message hashing here, instead
                // computing a Verihash digest of a message in `sequence::Iter`
                // then invoking the `hash_message_digest` method above.
                //
                // This code just handles length bookkeeping.
                if wire_type != WireType::Message || remaining - bytes.len() != new_remaining {
                    return Err(error::Kind::Hashing.into());
                }

                return Ok(State::Message {
                    remaining: new_remaining,
                });
            }
            _ => return Err(error::Kind::Hashing.into()),
        };

        verihash.update(bytes);
        Ok(new_state)
    }
}
