//! Fields (i.e. key/value pair) in a Veriform message

use crate::Error;
use core::convert::TryFrom;

/// Field headers
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Header {
    /// Tag which identifies the field
    pub tag: u64,

    /// Encoded value type for the field
    pub wire_type: WireType,
}

impl TryFrom<u64> for Header {
    type Error = Error;

    fn try_from(encoded: u64) -> Result<Self, Error> {
        let wire_type = WireType::try_from(encoded & 0b11)?;
        let tag = encoded >> 3;
        Ok(Header { tag, wire_type })
    }
}

/// Wire type identifiers for Veriform types
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum WireType {
    /// 64-bit unsigned integer
    UInt64 = 0,

    /// 64-bit (zigzag) signed integer
    SInt64 = 1,

    /// Nested Veriform message
    Message = 2,

    /// Binary data
    Bytes = 3,
}

impl TryFrom<u64> for WireType {
    type Error = Error;

    fn try_from(encoded: u64) -> Result<Self, Error> {
        match encoded {
            0 => Ok(WireType::UInt64),
            1 => Ok(WireType::SInt64),
            2 => Ok(WireType::Message),
            3 => Ok(WireType::Bytes),
            _ => Err(Error),
        }
    }
}
