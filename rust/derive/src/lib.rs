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
    /// Derive the `Message` trait for an `enum` or `struct`.
    ///
    /// When using this macro, every field in a `struct` or every variant of an
    /// `enum` must have one of the following attributes:
    ///
    /// - `#[field(...)]`: schema information for a field in a Veriform message
    /// - `#[digest(...)]`: (`struct` only) support for storing a computed
    ///   Verihash digest inside of a `struct` containing message fields.
    ///   The `digest` field MUST be the last in the message.
    ///
    /// See [`tests/derive.rs`] for usage examples.
    ///
    /// [`tests/derive.rs`]: https://github.com/iqlusioninc/veriform/blob/develop/rust/tests/derive.rs
    message::derive
);
