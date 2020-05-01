//! Universally unique identifiers.
//!
//! In Veriform these are encoded as:
//!
//! ```text
//! message UUID {
//!     value![0]: bytes(size = 16),
//! }
//! ```

pub use uuid::Uuid;

use crate::{
    decoder::{DecodeRef, Decoder},
    digest::Digest,
    field, Encoder, Error, Message,
};
use core::convert::TryInto;

impl Message for Uuid {
    fn decode<D>(decoder: &mut Decoder<D>, mut input: &[u8]) -> Result<Self, Error>
    where
        D: Digest,
    {
        let bytes: &[u8] = decoder.decode_ref(0, &mut input)?;

        bytes
            .try_into()
            .map(Uuid::from_bytes)
            .map_err(|_| Error::Builtin)
    }

    fn encode<'a>(&self, buffer: &'a mut [u8]) -> Result<&'a [u8], Error> {
        let mut encoder = Encoder::new(buffer);
        encoder.bytes(0, true, self.as_bytes())?;
        Ok(encoder.finish())
    }

    fn encoded_len(&self) -> usize {
        field::length::bytes(0, self.as_bytes())
    }
}
