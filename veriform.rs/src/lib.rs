//! veriform.rs: Cryptographically verifiable data serialization format
//! inspired by Protocol Buffers.

#![no_std]
#![doc(html_root_url = "https://docs.rs/vint64/0.0.0")]
#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

pub mod decoder;
pub mod field;

pub use crate::decoder::Decoder;

/// Error type
// TODO(tarcieri): capture more info?
#[derive(Debug)]
pub struct Error;
