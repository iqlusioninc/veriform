//! Veriform wire types

pub use crate::error::Error;
use core::convert::TryFrom;

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

    /// Sequences
    Sequence = 7,
}

impl WireType {
    /// Decode a [`WireType`] from an unmasked u64
    pub fn from_unmasked(value: u64) -> Result<Self, Error> {
        Self::try_from(value & 0b111)
    }

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
            7 => Ok(WireType::Sequence),
            _ => Err(Error::WireType),
        }
    }
}
