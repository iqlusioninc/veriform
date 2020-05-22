//! Sequence decoder

mod decoder;
mod hasher;
mod iter;
mod state;

pub use self::iter::Iter;

pub(crate) use self::decoder::Decoder;
