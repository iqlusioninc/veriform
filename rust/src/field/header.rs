//! Field headers

use super::{Tag, WireType};
use crate::error::Error;
use core::convert::TryFrom;
use vint64::VInt64;

/// Field headers
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Header {
    /// Tag which identifies the field
    pub tag: Tag,

    /// Encoded value type for the field
    pub wire_type: WireType,
}

impl Header {
    /// Encode this header value as a `Vint64`
    pub fn encode(self) -> VInt64 {
        vint64::encode(self.tag << 4 | self.wire_type as u64)
    }
}

impl TryFrom<u64> for Header {
    type Error = Error;

    fn try_from(encoded: u64) -> Result<Self, Error> {
        let wire_type = WireType::from_unmasked(encoded)?;
        let tag = encoded >> 4;
        Ok(Header { tag, wire_type })
    }
}
