//! TAI64(N) (Temps Atomique International) timestamps
//!
//! In Veriform these are encoded as:
//!
//! ```text
//! message TAI64N {
//!     secs[0]: !bytes(size = 8),
//!     nanos[1]: bytes(size = 4)
//! }
//! ```

pub use tai64::TAI64N;

use crate::{
    decoder::{Decodable, Decoder},
    encoder::Encoder,
    error::Error,
    field::{self, WireType},
    message::Message,
};
use core::convert::TryInto;

impl Message for TAI64N {
    fn decode(bytes: impl AsRef<[u8]>) -> Result<Self, Error> {
        let mut bytes = bytes.as_ref();
        let mut decoder = Decoder::new();

        decoder.decode_expected_header(&mut bytes, 0, WireType::UInt64)?;
        let secs = decoder.decode_uint64(&mut bytes)?;

        decoder.decode_expected_header(&mut bytes, 1, WireType::UInt64)?;
        let nanos = decoder.decode_uint64(&mut bytes)?;

        if nanos > core::u32::MAX as u64 {
            return Err(Error::Length);
        }

        let mut bytes = [0u8; 12];
        bytes[..8].copy_from_slice(&secs.to_le_bytes());
        bytes[8..].copy_from_slice(&(nanos as u32).to_le_bytes());
        bytes.try_into().map_err(|_| Error::Builtin)
    }

    fn encode<'a>(&self, buffer: &'a mut [u8]) -> Result<&'a mut [u8], Error> {
        let mut encoder = Encoder::new(buffer);
        let (secs, nanos) = tai64_to_ints(self);
        encoder.uint64(0, true, secs)?;
        encoder.uint64(1, false, nanos as u64)?;
        Ok(encoder.finish())
    }

    fn encoded_len(&self) -> usize {
        let (secs, nanos) = tai64_to_ints(self);
        field::length::uint64(0, secs) + field::length::uint64(1, nanos as u64)
    }
}

/// Convert a TAI64N timestamp to two integers
fn tai64_to_ints(tai64n: &TAI64N) -> (u64, u32) {
    let encoded = tai64n.to_bytes();
    let secs = u64::from_le_bytes(encoded[..8].try_into().unwrap());
    let nanos = u32::from_le_bytes(encoded[8..].try_into().unwrap());
    (secs, nanos)
}
