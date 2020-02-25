//! Field length calculations for various types

use super::{Header, Tag, WireType};
use crate::message::Message;
use vint64::VInt64;

/// Compute length of a `uint64` field
pub fn uint64(tag: Tag, critical: bool, value: u64) -> usize {
    header(tag, critical, WireType::UInt64) + VInt64::from(value).len()
}

/// Compute length of an `sint64` field
pub fn sint64(tag: Tag, critical: bool, value: i64) -> usize {
    header(tag, critical, WireType::SInt64) + VInt64::from(value).len()
}

/// Compute length of a `bytes` field
pub fn bytes(tag: Tag, critical: bool, bytes: &[u8]) -> usize {
    length_delimited(tag, critical, WireType::Bytes, bytes.len())
}

/// Compute length of a `message` field including the tag and delimiter
pub fn message(tag: Tag, critical: bool, message: &dyn Message) -> usize {
    length_delimited(tag, critical, WireType::Message, message.encoded_len())
}

/// Compute length of a `sequence` of `message` values including the tag and delimiter
pub fn message_seq<'a>(
    tag: Tag,
    critical: bool,
    messages: impl Iterator<Item = &'a dyn Message>,
) -> usize {
    let body_len: usize = messages.map(|msg| msg.encoded_len()).sum();
    header(tag, critical, WireType::Sequence)
        + VInt64::from((body_len as u64) << 4 | WireType::Message as u64).len()
        + body_len
}

/// Compute length of a field header
fn header(tag: Tag, critical: bool, wire_type: WireType) -> usize {
    Header::new(tag, critical, wire_type).encode().len()
}

/// Compute length of a length delimited field
fn length_delimited(tag: Tag, critical: bool, wire_type: WireType, length: usize) -> usize {
    header(tag, critical, wire_type) + VInt64::from(length as u64).len() + length
}
