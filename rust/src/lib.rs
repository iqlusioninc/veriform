//! Veriform: cryptographically verifiable data serialization format
//! inspired by Protocol Buffers.
//!
//! This crate provides a `no_std`-friendly implementation of the format
//! with a zero-copy pull parser.

#![no_std]
#![doc(html_root_url = "https://docs.rs/veriform/0.0.1")]
#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

#[cfg(feature = "alloc")]
#[macro_use]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(any(feature = "tai64", feature = "uuid"))]
pub mod builtins;
pub mod decoder;
pub mod encoder;
pub mod error;
pub mod field;
pub mod message;

pub use crate::{
    decoder::{Decodable, Decoder},
    encoder::Encoder,
    error::Error,
    message::Message,
};
pub use vint64;
