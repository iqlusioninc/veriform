//! Error type

use crate::{
    field::{Tag, WireType},
    message::Element,
};
use displaydoc::Display;

/// Error type
#[derive(Copy, Clone, Debug, Display, Eq, PartialEq)]
pub enum Error {
    /// decoding failed: wire_type={wire_type:?}
    Decode {
        /// element of the message that failed to decode
        element: Element,

        /// wire type we were looking for when decoding failed
        wire_type: WireType,
    },

    /// operation failed
    Failed,

    /// invalid field header: tag={tag:?} wire_type={wire_type:?}
    FieldHeader {
        /// tag which identifies this field
        tag: Option<Tag>,

        /// expected wire type for field
        wire_type: Option<WireType>,
    },

    /// bad length
    Length,

    /// field {tag:?} is out-of-order
    Order {
        /// tag of the out-of-order field
        tag: Tag,
    },

    /// unexpected trailing data
    TrailingData,

    /// truncated message: remaining={remaining:?} wire_type={wire_type:?}
    Truncated {
        /// number of bytes of remaining data expected in the message
        remaining: usize,

        /// wire type of the truncated data
        wire_type: WireType,
    },

    /// malformed UTF-8 encountered at byte: {valid_up_to:?}
    Utf8 {
        /// byte at which UTF-8 encoding failed
        valid_up_to: usize,
    },

    /// `vint64` encoding error
    VInt64,

    /// invalid wire type: {wanted:?}
    WireType {
        /// wire type we were looking for
        wanted: Option<WireType>,
    },
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
