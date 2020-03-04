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
    decoder::{Decodable, Decoder},
    encoder::Encoder,
    error::Error,
    field::{self, WireType},
    message::Message,
};
use core::convert::TryInto;

impl Message for Uuid {
    fn decode(bytes: impl AsRef<[u8]>) -> Result<Self, Error> {
        let mut bytes = bytes.as_ref();
        let mut decoder = Decoder::new();

        decoder.decode_expected_header(&mut bytes, 0, WireType::String)?;
        let uuid_bytes = decoder.decode_bytes(&mut bytes)?;

        uuid_bytes
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
