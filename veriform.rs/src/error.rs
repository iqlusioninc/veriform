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
            Error::WireType => write!(f, "invalid wire type"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
