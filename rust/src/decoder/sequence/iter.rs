//! Sequence iterator

use crate::{
    decoder::{sequence, Decodable, Decoder},
    field::Tag,
    Error, Message,
};
use core::marker::PhantomData;
use digest::Digest;

/// Sequence iterator: iterates over a sequence of values in a Veriform
/// message, decoding each one.
pub struct Iter<'a, 'b, T, D: Digest> {
    /// Sequence decoder
    decoder: &'a mut Decoder<D>,

    /// Tag for the field this sequence is contained in
    // TODO(tarcieri): support nested sequences?
    tag: Tag,

    /// Input data
    data: &'b [u8],

    /// Type to decode
    decodable: PhantomData<T>,
}

impl<'a, 'b, T, D> Iter<'a, 'b, T, D>
where
    D: Digest,
{
    /// Create a new sequence iterator from a sequence decoder
    pub(crate) fn new(decoder: &'a mut Decoder<D>, tag: Tag, data: &'b [u8]) -> Self {
        Self {
            decoder,
            tag,
            data,
            decodable: PhantomData,
        }
    }

    /// Borrow the sequence decoder
    fn seq_decoder(&mut self) -> &mut sequence::Decoder<D> {
        self.decoder.peek_seq()
    }
}

impl<'a, 'b, T, D> Iterator for Iter<'a, 'b, T, D>
where
    T: Message,
    D: Digest,
{
    type Item = Result<T, Error>;

    fn next(&mut self) -> Option<Result<T, Error>> {
        if self.seq_decoder().remaining() == 0 {
            return None;
        }

        let mut input = &self.data[self.seq_decoder().position()..];

        let result = self
            .seq_decoder()
            .decode_message(&mut input)
            .and_then(|msg_bytes| {
                self.decoder.push()?;
                let msg = T::decode(&mut self.decoder, msg_bytes)?;

                if let Some(digest) = self.decoder.pop() {
                    self.seq_decoder().hash_message_digest(&digest)?;
                }

                Ok(msg)
            });

        Some(result)
    }
}

impl<'a, 'b, D> Iterator for Iter<'a, 'b, u64, D>
where
    D: Digest,
{
    type Item = Result<u64, Error>;

    fn next(&mut self) -> Option<Result<u64, Error>> {
        if self.seq_decoder().remaining() == 0 {
            return None;
        }

        let mut input = &self.data[self.seq_decoder().position()..];
        Some(self.seq_decoder().decode_uint64(&mut input))
    }
}

impl<'a, 'b, D> Iterator for Iter<'a, 'b, i64, D>
where
    D: Digest,
{
    type Item = Result<i64, Error>;

    fn next(&mut self) -> Option<Result<i64, Error>> {
        if self.seq_decoder().remaining() == 0 {
            return None;
        }

        let mut input = &self.data[self.seq_decoder().position()..];
        Some(self.seq_decoder().decode_sint64(&mut input))
    }
}

impl<'a, 'b, T, D> Drop for Iter<'a, 'b, T, D>
where
    D: Digest,
{
    fn drop(&mut self) {
        if let Some(digest) = self.decoder.pop_seq() {
            self.decoder
                .peek()
                .hash_sequence_digest(self.tag, &digest)
                .unwrap();
        }
    }
}
