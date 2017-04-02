//! zser.rs: Implementation of the zcred serialization format

#![deny(missing_docs)]

#![cfg_attr(not(test), no_std)]
#![cfg_attr(feature = "bench", feature(test))]

extern crate byteorder;

#[cfg(all(feature = "bench", test))]
extern crate test;

// For competitive benchmarking
#[cfg(all(feature = "bench", test))]
extern crate leb128;

pub mod varint;
