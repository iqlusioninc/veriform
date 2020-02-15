//! Error type

use crate::field::Tag;
use core::fmt::{self, Display};

/// Error type
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    /// Malformed data encountered while decoding
    Decode,

    /// Operation failed or is in a bad state
    Failed,

    /// Length is incorrect/insufficient
    Length,

    /// Field is out-of-order
    Order {
        /// Tag of the out-of-order field
        tag: Tag,
    },

    /// String is not valid UTF-8
    Utf8 {
        /// Byte at which UTF-8 encoding failed
        valid_up_to: usize,
    },

    /// Invalid wire type
    WireType,
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Decode => write!(f, "decode error"),
            Error::Failed => write!(f, "operation failed"),
            Error::Length => write!(f, "bad length"),
            Error::Order { tag } => write!(f, "out-of-order field: {}", tag),
            Error::Utf8 { valid_up_to } => write!(f, "malformed UTF-8 at byte: {}", valid_up_to),
            Error::WireType => write!(f, "invalid wire type"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
