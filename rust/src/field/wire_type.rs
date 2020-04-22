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

    /// Binary data
    Bytes = 4,

    /// Unicode string
    String = 5,

    /// Nested Veriform message
    Message = 6,

    /// Sequences
    Sequence = 7,
}

impl WireType {
    /// Decode a [`WireType`] from an unmasked u64
    pub fn from_unmasked(value: u64) -> Self {
        // Never panics because all 3-bit wire types are valid
        Self::try_from(value & 0b111).unwrap()
    }

    /// Is this a dynamically-sized [`WireType`]?
    pub fn is_dynamically_sized(self) -> bool {
        match self {
            WireType::Bytes | WireType::String | WireType::Message | WireType::Sequence => true,
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
            4 => Ok(WireType::Bytes),
            5 => Ok(WireType::String),
            6 => Ok(WireType::Message),
            7 => Ok(WireType::Sequence),
            _ => Err(Error::InvalidWireType),
        }
    }
}
