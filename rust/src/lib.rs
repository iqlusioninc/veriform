//! Veriform: cryptographically verifiable data serialization format
//! inspired by Protocol Buffers.
//!
//! This crate provides a `no_std`-friendly implementation of the format
//! with a zero-copy pull parser.

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
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
mod verihash;

// Re-export the `digest` crate
pub use digest;

// Re-export the `vint64` crate
pub use vint64;

pub use crate::{encoder::Encoder, error::Error, message::Message};

/// Veriform decoder with the default SHA-256 hash
#[cfg(feature = "sha2")]
#[cfg_attr(docsrs, doc(cfg(feature = "sha2")))]
pub type Decoder = crate::decoder::Decoder<sha2::Sha256>;
