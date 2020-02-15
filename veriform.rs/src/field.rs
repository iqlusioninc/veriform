//! Fields (i.e. key/value pair) in a message

pub mod length;

use crate::Error;
use core::convert::TryFrom;
use vint64::VInt64;

/// Tag which identifies a field
pub type Tag = u64;

/// Field headers
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Header {
    /// Tag which identifies the field
    pub tag: Tag,

    /// Encoded value type for the field
    pub wire_type: WireType,
}

impl Header {
    /// Encode this header value as a `Vint64`
    pub fn encode(self) -> VInt64 {
        vint64::encode(self.tag << 3 | self.wire_type as u64)
    }
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
#[repr(u64)]
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

impl WireType {
    /// Is this a wiretype which is followed by a length delimiter?
    pub fn is_length_delimited(self) -> bool {
        match self {
            WireType::Message | WireType::Bytes => true,
            _ => false,
        }
    }
}

impl TryFrom<u64> for WireType {
    type Error = Error;

    fn try_from(encoded: u64) -> Result<Self, Error> {
        match encoded {
            0 => Ok(WireType::UInt64),
            1 => Ok(WireType::SInt64),
            2 => Ok(WireType::Message),
            3 => Ok(WireType::Bytes),
            _ => Err(Error::WireType),
        }
    }
}
