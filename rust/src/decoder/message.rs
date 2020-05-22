//! Veriform message decoder

mod body;
mod decoder;
mod hasher;
mod header;
mod state;
mod value;

pub(crate) use self::decoder::Decoder;
