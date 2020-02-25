//! Events emitted by Veriform's decoder

use crate::field::{Header, WireType};

/// Events emitted by Veriform's decoder
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Event<'a> {
    /// Consumed field header with the given tag and wire type
    FieldHeader(Header),

    /// Consumed a boolean value
    Bool(bool),

    /// Consumed an unsigned 64-bit integer
    UInt64(u64),

    /// Consumed a signed 64-bit integer
    SInt64(i64),

    /// Consumed a length delimiter for the given wire type
    LengthDelimiter {
        /// Wire type of the value this length delimits
        wire_type: WireType,

        /// Length of the field body (sans delimiter)
        length: usize,
    },

    /// Consumed a chunk of a length-delimited value
    ValueChunk {
        /// Wire type of the value being consumed
        wire_type: WireType,

        /// Bytes in this chunk
        bytes: &'a [u8],

        /// Remaining bytes in the message
        remaining: usize,
    },

    /// Consumed the header of a sequence of the given wiretype
    SequenceHeader {
        /// Wire type contained in this sequence
        wire_type: WireType,

        /// Length of the sequence body
        length: usize,
    },
}
