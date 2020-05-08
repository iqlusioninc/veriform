//! Veriform message decoder

mod body;
mod decoder;
mod hasher;
mod header;
mod state;
mod value;

pub use self::decoder::Decoder;
