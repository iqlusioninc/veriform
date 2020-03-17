//! Error type

use core::fmt::{self, Display};

/// Error type
#[derive(Copy, Clone, Debug)]
pub enum Error {
    /// Value contains unnecessary leading zeroes
    LeadingZeroes,

    /// Value is truncated / malformed
    Truncated,
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Error::LeadingZeroes => "leading zeroes in vint64 value",
            Error::Truncated => "truncated vint64 value",
        })
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
