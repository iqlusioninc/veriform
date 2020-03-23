//! Support for encoding signed integers as `vint64`.

use crate::{Error, VInt64};

/// Encode a signed integer as a zigzag-encoded `vint64`.
#[inline]
pub fn encode(value: i64) -> VInt64 {
    value.into()
}

/// Decode a zigzag-encoded `vint64` as a signed integer.
#[inline]
pub fn decode(input: &mut &[u8]) -> Result<i64, Error> {
    super::decode(input).map(zigzag::decode)
}

/// Get the length of a zigzag encoded `vint64` for the given value in bytes.
#[inline]
pub fn encoded_len(value: i64) -> usize {
    super::encoded_len(zigzag::encode(value))
}

/// Zigzag encoding for signed integers.
///
/// This module contains the raw zigzag encoding algorithm.
///
/// For encoding signed integers as `vint64`, use the functions located in
/// the parent [`vint64::signed`](../index.html) module.
pub mod zigzag {
    /// Encode a signed 64-bit integer in zigzag encoding
    #[inline]
    pub fn encode(value: i64) -> u64 {
        ((value << 1) ^ (value >> 63)) as u64
    }

    /// Decode a signed 64-bit integer from zigzag encoding
    #[inline]
    pub fn decode(encoded: u64) -> i64 {
        (encoded >> 1) as i64 ^ -((encoded & 1) as i64)
    }
}
