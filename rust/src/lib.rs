//! zser.rs: Implementation of the zcred serialization format

#![crate_name = "zser"]
#![crate_type = "lib"]

#![deny(missing_docs)]

// For error-chain
#![recursion_limit = "1024"]

#![cfg_attr(not(any(feature = "std", test)), no_std)]
#![cfg_attr(not(feature = "std"), feature(collections))]
#![cfg_attr(feature = "bench", feature(test))]

extern crate byteorder;

#[cfg(not(feature = "std"))]
#[macro_use]
extern crate collections;

#[cfg(feature = "std")]
extern crate core;

#[cfg(all(feature = "bench", test))]
extern crate test;

// For competitive benchmarking
#[cfg(all(feature = "bench", test))]
extern crate leb128;

pub mod decoder;
pub mod errors;
pub mod parser;
pub mod value;
pub mod varint;

pub use value::Value;
