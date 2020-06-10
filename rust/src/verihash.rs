//! Verihash core hashing primitives

// TODO(tarcieri): refactor/DRY out message/sequence hashers into this module

use crate::field::{Tag, WireType};
use digest::{generic_array::GenericArray, Digest};

/// Output of a given digest algorithm
pub type DigestOutput<D> = GenericArray<u8, <D as Digest>::OutputSize>;

/// Verihash prefix used by tags (unsigned integer)
// TODO(tarcieri): support string tags?
const TAG_PREFIX: u8 = WireType::UInt64.to_u8();

/// Verihash hasher: computes digests of both messages and sequences
pub(crate) struct Hasher<D: Digest>(D);

impl<D> Hasher<D>
where
    D: Digest,
{
    /// Create a new Verihash hasher
    pub fn new() -> Self {
        Hasher(D::new())
    }

    /// Hash a tagged boolean value
    pub fn tagged_boolean(&mut self, tag: Tag, value: bool) {
        self.tag(tag);
        self.boolean(value);
    }

    /// Hash a boolean
    pub fn boolean(&mut self, value: bool) {
        if value {
            self.fixed_size_value(WireType::True, b"\x01")
        } else {
            self.fixed_size_value(WireType::False, b"\x00")
        }
    }

    /// Hash a tagged unsigned 64-bit integer
    pub fn tagged_uint64(&mut self, tag: Tag, value: u64) {
        self.tag(tag);
        self.uint64(value);
    }

    /// Hash an unsigned 64-bit integer
    pub fn uint64(&mut self, value: u64) {
        self.fixed_size_value(WireType::UInt64, &value.to_le_bytes());
    }

    /// Hash a tagged signed 64-bit integer
    pub fn tagged_sint64(&mut self, tag: Tag, value: i64) {
        self.tag(tag);
        self.sint64(value);
    }

    /// Hash a signed 64-bit integer
    pub fn sint64(&mut self, value: i64) {
        self.fixed_size_value(WireType::SInt64, &value.to_le_bytes());
    }

    /// Hash a numerical tag
    // TODO(tarcieri): support string tags?
    pub fn tag(&mut self, tag: Tag) {
        self.update(&[TAG_PREFIX]);
        self.update(&tag.to_le_bytes());
    }

    /// Hash a dynamically sized value
    pub fn dynamically_sized_value(&mut self, wire_type: WireType, length: usize) {
        self.update(&[wire_type.to_u8()]);
        self.update(&(length as u64).to_le_bytes());
    }

    /// Hash an untagged value
    pub fn fixed_size_value(&mut self, wire_type: WireType, body: &[u8]) {
        self.update(&[wire_type.to_u8()]);
        self.update(body);
    }

    /// Update data directly into the underlying hash function
    pub fn update(&mut self, data: &[u8]) {
        self.0.update(data);
    }

    /// Finish computing the digest, returning the output value
    pub fn finalize(self) -> DigestOutput<D> {
        self.0.finalize()
    }
}

impl<D> Default for Hasher<D>
where
    D: Digest,
{
    fn default() -> Self {
        Self::new()
    }
}
