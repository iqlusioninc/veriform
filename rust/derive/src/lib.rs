//! Custom derive for Veriform `Message` trait.
//!
//! # Development Notes
//!
//! To see code generated using these proc macros, install `cargo expand` and
//! with the nightly Rust compiler, run the following:
//!
//! ```text
//! $ cargo expand --test derive
//! ```

#![crate_type = "proc-macro"]
#![forbid(unsafe_code)]
#![warn(rust_2018_idioms)]

mod digest;
mod field;
mod message;

use synstructure::decl_derive;

decl_derive!(
    [Message, attributes(digest, field)] =>
    /// Derive the `Message` trait for an enum or struct
    message::derive
);
