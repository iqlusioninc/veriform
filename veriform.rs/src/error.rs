//! Error type

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

    /// Invalid wire type
    WireType,
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Error::Decode => "decode error",
            Error::Failed => "operation failed",
            Error::Length => "bad length",
            Error::WireType => "invalid wire type",
        })
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
