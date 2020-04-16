//! TAI64(N) (Temps Atomique International) timestamps
//!
//! In Veriform these are encoded as:
//!
//! ```text
//! message Timestamp {
//!     secs[0]: !bytes(size = 8),
//!     nanos[1]: bytes(size = 4)
//! }
//! ```

pub use tai64::TAI64N as Timestamp;

use crate::{decoder::Decode, field, Decoder, Encoder, Error, Message};
use core::convert::TryInto;

impl Message for Timestamp {
    fn decode(decoder: &mut Decoder, mut input: &[u8]) -> Result<Self, Error> {
        let secs: u64 = decoder.decode(0, &mut input)?;
        let nanos: u64 = decoder.decode(1, &mut input)?;

        if nanos > core::u32::MAX as u64 {
            return Err(Error::Length);
        }

        let mut bytes = [0u8; 12];
        bytes[..8].copy_from_slice(&secs.to_le_bytes());
        bytes[8..].copy_from_slice(&(nanos as u32).to_le_bytes());
        bytes.try_into().map_err(|_| Error::Builtin)
    }

    fn encode<'a>(&self, buffer: &'a mut [u8]) -> Result<&'a [u8], Error> {
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

/// Convert a Timestamp timestamp to two integers
fn tai64_to_ints(tai64n: &Timestamp) -> (u64, u32) {
    let encoded = tai64n.to_bytes();
    let secs = u64::from_le_bytes(encoded[..8].try_into().unwrap());
    let nanos = u32::from_le_bytes(encoded[8..].try_into().unwrap());
    (secs, nanos)
}
