//! Field headers

use super::{Tag, WireType};
use vint64::VInt64;

/// Field headers
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Header {
    /// Tag which identifies the field
    pub tag: Tag,

    /// Indicates a field containing critical information which
    /// shouldn't be ignored if the field is unknown
    pub critical: bool,

    /// Encoded value type for the field
    pub wire_type: WireType,
}

impl Header {
    /// Create a new header
    pub fn new(tag: Tag, critical: bool, wire_type: WireType) -> Self {
        Header {
            tag,
            critical,
            wire_type,
        }
    }

    /// Encode this header value as a `vint64`
    pub fn encode(self) -> VInt64 {
        vint64::encode(self.into())
    }

    /// Get the length of this header when encoded as `vint64`
    pub fn encoded_len(self) -> usize {
        vint64::encoded_len(self.into())
    }
}

impl From<u64> for Header {
    fn from(encoded: u64) -> Self {
        Header {
            tag: encoded >> 4,
            critical: encoded >> 3 & 1 == 1,
            wire_type: WireType::from_unmasked(encoded),
        }
    }
}

impl From<Header> for u64 {
    fn from(header: Header) -> u64 {
        header.tag << 4 | (header.critical as u64) << 3 | header.wire_type as u64
    }
}
