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
        let wire_type = WireType::try_from(encoded & 0b111)?;
        let tag = encoded >> 3;
        Ok(Header { tag, wire_type })
    }
}

/// Wire type identifiers for Veriform types
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u64)]
pub enum WireType {
    /// False (boolean)
    False = 0,

    /// True (boolean)
    True = 1,

    /// 64-bit unsigned integer
    UInt64 = 2,

    /// 64-bit (zigzag) signed integer
    SInt64 = 3,

    /// Nested Veriform message
    Message = 4,

    /// Binary data
    Bytes = 5,

    /// Unicode string
    String = 6,
}

impl WireType {
    /// Is this a wiretype which is followed by a length delimiter?
    pub fn is_length_delimited(self) -> bool {
        match self {
            WireType::Message | WireType::Bytes | WireType::String => true,
            _ => false,
        }
    }
}

impl TryFrom<u64> for WireType {
    type Error = Error;

    fn try_from(encoded: u64) -> Result<Self, Error> {
        match encoded {
            0 => Ok(WireType::False),
            1 => Ok(WireType::True),
            2 => Ok(WireType::UInt64),
            3 => Ok(WireType::SInt64),
            4 => Ok(WireType::Message),
            5 => Ok(WireType::Bytes),
            6 => Ok(WireType::String),
            _ => Err(Error::WireType),
        }
    }
}
