//! Integration tests for `veriform_derive`

use heapless::{
    consts::{U1024, U8},
    Vec,
};
use veriform::{Decoder, Message};

/// Buffer type.
///
/// Using `heapless::Vec` lets us test on `no_std`.
type Buffer = Vec<u8, U1024>;

/// Create a new buffer for encoding tests
pub fn new_buffer() -> Buffer {
    let mut vec = Vec::new();
    vec.extend_from_slice(&[0u8; 1024]).unwrap();
    vec
}

#[derive(Message, Debug, Eq, PartialEq)]
pub enum ExampleEnum {
    #[field(tag = 0, wire_type = "bytes", size = 32)]
    BytesVariant([u8; 32]),
}

impl Default for ExampleEnum {
    fn default() -> Self {
        Self::BytesVariant([
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31,
        ])
    }
}

#[test]
fn enum_round_trip() {
    let example = ExampleEnum::default();

    let mut encoded_buf = new_buffer();
    let encoded_len = example.encode(&mut encoded_buf).unwrap().len();
    encoded_buf.truncate(encoded_len);

    let mut decoder = Decoder::new();
    let decoded = ExampleEnum::decode(&mut decoder, &encoded_buf).unwrap();

    assert_eq!(example, decoded);
}

#[derive(Message, Debug, Eq, PartialEq)]
pub struct ExampleStruct {
    #[field(tag = 0, wire_type = "uint64", critical = true)]
    pub uint64_field: u64,

    #[field(tag = 1, wire_type = "sint64", critical = true)]
    pub sint64_field: i64,

    #[field(tag = 2, wire_type = "sequence", critical = true, max = 8)]
    pub msg_sequence_field: heapless::Vec<ExampleEnum, U8>,
}

impl Default for ExampleStruct {
    fn default() -> Self {
        let mut msg_sequence_field = heapless::Vec::new();

        for _ in 0..3 {
            msg_sequence_field.push(ExampleEnum::default()).unwrap();
        }

        Self {
            uint64_field: 42,
            sint64_field: -42,
            msg_sequence_field,
        }
    }
}

#[test]
fn struct_round_trip() {
    let example = ExampleStruct::default();

    let mut encoded_buf = new_buffer();
    let encoded_len = example.encode(&mut encoded_buf).unwrap().len();
    encoded_buf.truncate(encoded_len);

    let mut decoder = Decoder::new();
    let decoded = ExampleStruct::decode(&mut decoder, &encoded_buf).unwrap();

    assert_eq!(example, decoded);
}
