//! Field length calculations for various types

use super::{Header, Tag, WireType};
use crate::message::Message;

/// Compute length of a `uint64` field
pub fn uint64(tag: Tag, value: u64) -> usize {
    header(tag, WireType::UInt64) + vint64::encoded_len(value)
}

/// Compute length of an `sint64` field
pub fn sint64(tag: Tag, value: i64) -> usize {
    header(tag, WireType::SInt64) + vint64::signed::encoded_len(value)
}

/// Compute length of a `bytes` field
pub fn bytes(tag: Tag, bytes: &[u8]) -> usize {
    dynamically_sized(tag, WireType::Bytes, bytes.len())
}

/// Compute length of a `string` field
pub fn string(tag: Tag, string: &str) -> usize {
    dynamically_sized(tag, WireType::String, string.len())
}

/// Compute length of a `message` field including the tag and delimiter
pub fn message(tag: Tag, message: &dyn Message) -> usize {
    dynamically_sized(tag, WireType::Message, message.encoded_len())
}

/// Compute length of a `sequence` of `message` values including the tag and delimiter
pub fn message_seq<'a>(tag: Tag, messages: impl Iterator<Item = &'a dyn Message>) -> usize {
    let body_len: usize = messages
        .map(|msg| {
            let encoded_len = msg.encoded_len();
            vint64::encoded_len(encoded_len as u64) + encoded_len
        })
        .sum();

    header(tag, WireType::Sequence)
        + vint64::encoded_len((body_len as u64) << 4 | WireType::Message as u64)
        + body_len
}

/// Compute length of a field header
fn header(tag: Tag, wire_type: WireType) -> usize {
    // Note: there shouldn't be any cases where the critical bit affects length
    Header::new(tag, false, wire_type).encoded_len()
}

/// Compute length of a dynamically sized field
fn dynamically_sized(tag: Tag, wire_type: WireType, length: usize) -> usize {
    header(tag, wire_type) + vint64::encoded_len(length as u64) + length
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uint64_length() {
        assert_eq!(uint64(1, 42), 2);
    }

    #[test]
    fn sint64_length() {
        assert_eq!(sint64(2, -42), 2);
    }

    #[test]
    fn bytes_length() {
        assert_eq!(bytes(3, b"foobar"), 8)
    }

    #[test]
    fn string_length() {
        assert_eq!(string(4, "baz"), 5);
    }
}
