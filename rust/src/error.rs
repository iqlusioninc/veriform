//! Error types

use crate::{
    field::{Tag, WireType},
    message::Element,
};
use core::fmt::{self, Display};
use displaydoc::Display;

/// Error type
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Error {
    /// Kind of error
    kind: Kind,

    /// Position inside of message where error occurred
    position: Option<usize>,
}

impl Error {
    /// Get the [`Kind`] of error that occurred
    pub fn kind(self) -> Kind {
        self.kind
    }

    /// Get the position inside of the message where the error occurred
    /// (if available)
    ///
    /// NOTE: support for this is an unreliable work-in-progress. Most of the
    /// time this will return `None`.
    pub fn position(self) -> Option<usize> {
        self.position
    }

    /// Extend the position within a message (for nested messages)
    // TODO(tarcieri): remove `#[allow(dead_code)]` attrs once this method is used
    #[allow(dead_code)]
    pub(crate) fn extend_position(self, pos: usize) -> Self {
        let new_position = self.position.map(|old_pos| old_pos + pos).unwrap_or(pos);

        Self {
            kind: self.kind,
            position: Some(new_position),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)?;

        if let Some(pos) = self.position {
            write!(f, " position={}", pos)?;
        }

        Ok(())
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

/// Kinds of errors
#[derive(Copy, Clone, Debug, Display, Eq, PartialEq)]
pub enum Kind {
    /// error decoding builtin type
    Builtin,

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

    /// hashing operation failed
    // TODO(tarcieri): collect more info
    Hashing,

    /// invalid wire type
    InvalidWireType,

    /// bad length
    Length,

    /// maximum message nesting depth exceeded
    NestingDepth,

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

    /// unexpected wire type: actual={actual:?} wanted={wanted:?}
    UnexpectedWireType {
        /// actual wire type found in message
        actual: WireType,

        /// wire type we were looking for
        wanted: WireType,
    },

    /// malformed UTF-8 encountered at byte: {valid_up_to:?}
    Utf8 {
        /// byte at which UTF-8 encoding failed
        valid_up_to: usize,
    },

    /// `vint64` encoding error
    VInt64,
}

impl Kind {
    /// Create an error with the given position
    pub(crate) fn position(self, pos: usize) -> Error {
        Error {
            kind: self,
            position: Some(pos),
        }
    }
}

impl From<Kind> for Error {
    fn from(kind: Kind) -> Self {
        Self {
            kind,
            position: None,
        }
    }
}
