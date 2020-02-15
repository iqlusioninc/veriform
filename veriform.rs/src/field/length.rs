//! Field length calculations for various types

use super::{Header, Tag, WireType};
use crate::message::Message;
use vint64::VInt64;

/// Compute length of a `uint64` field
pub fn uint64(tag: Tag, value: u64) -> usize {
    header(tag, WireType::UInt64) + VInt64::from(value).len()
}

/// Compute length of an `sint64` field
pub fn sint64(tag: Tag, value: i64) -> usize {
    header(tag, WireType::SInt64) + VInt64::from(value).len()
}

/// Compute length of a `bytes` field
pub fn bytes(tag: Tag, bytes: &[u8]) -> usize {
    length_delimited(tag, WireType::Bytes, bytes.len())
}

/// Compute length of a nested `message` field including the tag and delimiter
pub fn message<M: Message>(tag: Tag, message: M) -> usize {
    length_delimited(tag, WireType::Message, message.encoded_len())
}

/// Compute length of a field header
fn header(tag: Tag, wire_type: WireType) -> usize {
    Header { tag, wire_type }.encode().len()
}

/// Compute length of a length delimited field
fn length_delimited(tag: Tag, wire_type: WireType, length: usize) -> usize {
    header(tag, wire_type) + VInt64::from(length as u64).len() + length
}
