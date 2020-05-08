//! Sequence iterator

use super::Decoder;
use crate::{decoder::Decodable, Error, Message};
use core::marker::PhantomData;
use digest::Digest;

/// Sequence iterator: iterates over a sequence of values in a Veriform
/// message, decoding each one.
pub struct Iter<'a, T, D: Digest> {
    /// Sequence decoder
    decoder: Decoder<D>,

    /// Input data
    data: &'a [u8],

    /// Type to decode
    decodable: PhantomData<T>,
}

impl<'a, T, D> Iter<'a, T, D>
where
    D: Digest,
{
    /// Create a new sequence iterator from a sequence decoder
    pub(crate) fn new(decoder: Decoder<D>, data: &'a [u8]) -> Self {
        Self {
            decoder,
            data,
            decodable: PhantomData,
        }
    }
}

impl<'a, T, D> Iterator for Iter<'a, T, D>
where
    T: Message,
    D: Digest,
{
    type Item = Result<T, Error>;

    fn next(&mut self) -> Option<Result<T, Error>> {
        if self.decoder.remaining() == 0 {
            return None;
        }

        let mut input = &self.data[self.decoder.position()..];

        // TODO(tarcieri): reuse decoder!
        let mut decoder: crate::decoder::Decoder<D> = crate::decoder::Decoder::new();

        let result = self
            .decoder
            .decode_message(&mut input)
            .and_then(|msg_bytes| T::decode(&mut decoder, msg_bytes));

        Some(result)
    }
}

impl<'a, D> Iterator for Iter<'a, u64, D>
where
    D: Digest,
{
    type Item = Result<u64, Error>;

    fn next(&mut self) -> Option<Result<u64, Error>> {
        if self.decoder.remaining() == 0 {
            return None;
        }

        let mut input = &self.data[self.decoder.position()..];
        Some(self.decoder.decode_uint64(&mut input))
    }
}

impl<'a, D> Iterator for Iter<'a, i64, D>
where
    D: Digest,
{
    type Item = Result<i64, Error>;

    fn next(&mut self) -> Option<Result<i64, Error>> {
        if self.decoder.remaining() == 0 {
            return None;
        }

        let mut input = &self.data[self.decoder.position()..];
        Some(self.decoder.decode_sint64(&mut input))
    }
}
