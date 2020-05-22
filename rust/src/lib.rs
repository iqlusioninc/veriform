//! Veriform: cryptographically verifiable data serialization format
//! inspired by Protocol Buffers.
//!
//! This crate provides a `no_std`-friendly implementation of the format
//! with a zero-copy pull parser.
//!
//! For more information on Veriform, see the work-in-progress specification:
//! <https://github.com/iqlusioninc/veriform/blob/develop/spec/draft-veriform-spec.md>
//!
//! # Usage
//!
//! The main API for encoding and decoding Veriform messages is the
//! [`Message`] trait. When the `veriform_derive` feature of this crate
//! is enabled, custom derive is available for this trait for both structs
//! and enums.
//!
//! # Built-in Types
//!
//! Veriform has a small "standard library" of so-called "built-in types" which
//! are serialized using message syntax, but in a consistent way which allows
//! different programming language environments to use the best-available
//! native representation for these types.
//!
//! - [`Timestamp`]: date/time as represented in International Atomic Time (TAI)
//! - [`Uuid`]: universally unique identifier
//!
//! [`Timestamp`]: https://docs.rs/veriform/latest/veriform/builtins/struct.Timestamp.html
//! [`Uuid`]: https://docs.rs/veriform/latest/veriform/builtins/struct.Uuid.html

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(html_root_url = "https://docs.rs/veriform/0.2.0")]
#![forbid(unsafe_code)]
#![warn(
    missing_docs,
    rust_2018_idioms,
    unused_qualifications,
    intra_doc_link_resolution_failure
)]

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
mod string;
mod verihash;

#[cfg(feature = "veriform_derive")]
pub mod derive_helpers;

// Re-export the `digest` crate
pub use digest;

// Re-export the `vint64` crate
pub use vint64;

pub use crate::{encoder::Encoder, error::Error, message::Message};

/// Veriform decoder with the default SHA-256 hash
#[cfg(feature = "sha2")]
#[cfg_attr(docsrs, doc(cfg(feature = "sha2")))]
pub type Decoder = crate::decoder::Decoder<sha2::Sha256>;

/// SHA-256 digests
#[cfg(feature = "sha2")]
#[cfg_attr(docsrs, doc(cfg(feature = "sha2")))]
pub type Sha256Digest = [u8; 32];

#[cfg(feature = "veriform_derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "veriform_derive")))]
pub use veriform_derive::Message;
