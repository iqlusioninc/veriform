//! Error type

use crate::field::Tag;
use displaydoc::Display;

/// Error type
#[derive(Copy, Clone, Debug, Display, Eq, PartialEq)]
pub enum Error {
    /// decode error
    Decode,

    /// operation failed
    Failed,

    /// bad length
    Length,

    /// field {tag:?} is out-of-order
    Order {
        /// tag of the out-of-order field
        tag: Tag,
    },

    /// malformed UTF-8 encountered at byte: {valid_up_to:?}
    Utf8 {
        /// byte at which UTF-8 encoding failed
        valid_up_to: usize,
    },

    /// invalid wire type
    WireType,
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
