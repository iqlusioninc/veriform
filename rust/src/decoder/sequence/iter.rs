//! Sequence iterator

use super::Decoder;
use crate::{decoder::Decodable, Error, Message};
use core::marker::PhantomData;

// TODO(tarcieri): make this (and `sequence::Decoder`) generic around digest
#[cfg(not(feature = "sha2"))]
compile_error!("TODO: support disabling `sha2` feature");

/// Sequence iterator: iterates over a sequence of values in a Veriform
/// message, decoding each one.
pub struct Iter<'a, T> {
    /// Sequence decoder
    decoder: Decoder,

    /// Input data
    data: &'a [u8],

    /// Type to decode
    decodable: PhantomData<T>,
}

impl<'a, T> Iter<'a, T> {
    /// Create a new sequence iterator from a sequence decoder
    pub(crate) fn new(decoder: Decoder, data: &'a [u8]) -> Self {
        Self {
            decoder,
            data,
            decodable: PhantomData,
        }
    }
}

impl<'a, T: Message> Iterator for Iter<'a, T> {
    type Item = Result<T, Error>;

    fn next(&mut self) -> Option<Result<T, Error>> {
        if self.decoder.remaining() == 0 {
            return None;
        }

        let mut input = &self.data[self.decoder.position()..];

        // TODO(tarcieri): reuse decoder; support disabling `sha2` feature
        let mut decoder = crate::Decoder::new();

        let result = self
            .decoder
            .decode_message(&mut input)
            .and_then(|msg_bytes| T::decode(&mut decoder, msg_bytes));

        Some(result)
    }
}

impl<'a> Iterator for Iter<'a, u64> {
    type Item = Result<u64, Error>;

    fn next(&mut self) -> Option<Result<u64, Error>> {
        if self.decoder.remaining() == 0 {
            return None;
        }

        let mut input = &self.data[self.decoder.position()..];
        Some(self.decoder.decode_uint64(&mut input))
    }
}

impl<'a> Iterator for Iter<'a, i64> {
    type Item = Result<i64, Error>;

    fn next(&mut self) -> Option<Result<i64, Error>> {
        if self.decoder.remaining() == 0 {
            return None;
        }

        let mut input = &self.data[self.decoder.position()..];
        Some(self.decoder.decode_sint64(&mut input))
    }
}
