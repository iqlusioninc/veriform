//! veriform.rs: Cryptographically verifiable data serialization format
//! inspired by Protocol Buffers

#![cfg_attr(not(any(feature = "std", test)), no_std)]
#![cfg_attr(not(feature = "std"), feature(alloc))]
#![cfg_attr(feature = "bench", feature(test))]
#![warn(missing_docs, warnings)]

extern crate byteorder;

#[cfg(not(feature = "std"))]
#[macro_use]
extern crate alloc;

#[cfg(feature = "std")]
extern crate core;

#[cfg(all(feature = "bench", test))]
extern crate test;

pub mod decoder;
pub mod errors;
pub mod parser;
pub mod value;
pub mod varint;

pub use value::Value;
