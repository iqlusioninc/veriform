//! Verihash core hashing primitives

use crate::field::{Tag, WireType};
use digest::Digest;

/// Verihash prefix used by tags (unsigned integer)
// TODO(tarcieri): support string tags?
const TAG_PREFIX: u8 = WireType::UInt64.to_u8();

/// Hash a boolean
pub fn hash_boolean<D: Digest>(digest: &mut D, tag: Tag, value: bool) {
    let (wire_type, body) = if value {
        (WireType::True, b"\x01")
    } else {
        (WireType::False, b"\x00")
    };

    hash_tag(digest, tag);
    hash_fixed(digest, wire_type, body);
}

/// Hash an unsigned integer
pub fn hash_uint64<D: Digest>(digest: &mut D, tag: Tag, value: u64) {
    hash_tag(digest, tag);
    hash_fixed(digest, WireType::UInt64, &value.to_le_bytes());
}

/// Hash a signed integer
pub fn hash_sint64<D: Digest>(digest: &mut D, tag: Tag, value: i64) {
    hash_tag(digest, tag);
    hash_fixed(digest, WireType::SInt64, &value.to_le_bytes());
}

/// Hash a numerical tag
// TODO(tarcieri): support string tags?
pub fn hash_tag<D: Digest>(digest: &mut D, tag: Tag) {
    digest.input(&[TAG_PREFIX]);
    digest.input(&tag.to_le_bytes());
}

/// Hash a dynamically sized value
pub fn hash_dynamically_sized_value<D: Digest>(digest: &mut D, wire_type: WireType, length: usize) {
    digest.input(&[wire_type.to_u8()]);
    digest.input(&(length as u64).to_le_bytes());
}

/// Hash an untagged value
pub fn hash_fixed<D: Digest>(digest: &mut D, wire_type: WireType, body: &[u8]) {
    digest.input(&[wire_type.to_u8()]);
    digest.input(body);
}
