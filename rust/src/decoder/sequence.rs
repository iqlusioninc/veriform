//! Sequence decoder

mod decoder;
mod hasher;
mod iter;
mod state;

pub use self::{decoder::Decoder, iter::Iter};
