//! Veriform decoder

pub(crate) mod message;
pub mod sequence;

mod decodable;
mod event;
mod traits;
mod vint64;

#[cfg(feature = "log")]
#[macro_use]
mod trace;

pub use self::{
    decodable::Decodable,
    event::Event,
    traits::{Decode, DecodeRef, DecodeSeq},
};

use crate::{
    error::{self, Error},
    field::{Tag, WireType},
    Message,
};
use digest::{generic_array::GenericArray, Digest};
use heapless::consts::U16;

/// Veriform decoder
pub struct Decoder<D: Digest> {
    /// Stack of message decoders (max nesting depth 16)
    stack: heapless::Vec<message::Decoder<D>, U16>,
}

impl<D> Default for Decoder<D>
where
    D: Digest,
{
    fn default() -> Self {
        let mut stack = heapless::Vec::new();
        stack.push(message::Decoder::new()).unwrap();
        Decoder { stack }
    }
}

impl<D> Decoder<D>
where
    D: Digest,
{
    /// Initialize decoder
    pub fn new() -> Self {
        Self::default()
    }

    /// Push a new message decoder down onto the stack
    // TODO(tarcieri): higher-level API (more like `::decode_message`)
    pub fn push(&mut self) -> Result<(), Error> {
        self.stack
            .push(message::Decoder::new())
            .map_err(|_| error::Kind::NestingDepth.into())
    }

    /// Pop the message decoder from the stack when we've finished a message.
    ///
    /// Returns a digest of the nested message if message hashing is enabled.
    ///
    /// Panics if the decoder's stack underflows.
    // TODO(tarcieri): panic-free higher-level API, possibly RAII-based?
    pub fn pop(&mut self) -> Option<GenericArray<u8, D::OutputSize>> {
        self.stack.pop().unwrap().compute_digest().unwrap()
    }

    /// Peek at the message decoder on the top of the stack
    // TODO(tarcieri): remove this implementation detail from public API
    pub fn peek(&mut self) -> &mut message::Decoder<D> {
        self.stack.last_mut().unwrap()
    }

    /// Get the depth of the pushdown stack
    // TODO(tarcieri): remove this implementation detail from public API
    pub fn depth(&self) -> usize {
        self.stack.len()
    }

    /// Fill the provided slice with the digest of the message if it fits
    // TODO(tarcieri): find a better way to handle generic digest sizes
    pub fn fill_digest(&mut self, output: &mut [u8]) -> Result<(), Error> {
        let digest = self
            .peek()
            .compute_digest()?
            .ok_or_else(|| error::Kind::Hashing)?;

        if digest.len() != output.len() {
            return Err(error::Kind::Hashing)?;
        }

        output.copy_from_slice(&digest);
        Ok(())
    }
}

impl<D, M> Decode<M> for Decoder<D>
where
    D: Digest,
    M: Message,
{
    fn decode(&mut self, tag: Tag, input: &mut &[u8]) -> Result<M, Error> {
        #[cfg(feature = "log")]
        begin!(self, "[{}]: msg?", tag);

        self.peek().expect_header(input, tag, WireType::Message)?;
        let msg_bytes = self.peek().decode_message(input)?;

        self.push()?;
        let msg = M::decode(self, msg_bytes)?;

        if let Some(digest) = self.pop() {
            self.peek().hash_message_digest(tag, &digest)?;
        }

        Ok(msg)
    }
}

impl<D> Decode<u64> for Decoder<D>
where
    D: Digest,
{
    fn decode(&mut self, tag: Tag, input: &mut &[u8]) -> Result<u64, Error> {
        #[cfg(feature = "log")]
        begin!(self, "[{}]: uint64?", tag);

        self.peek().expect_header(input, tag, WireType::UInt64)?;
        self.peek().decode_uint64(input)
    }
}

impl<D> Decode<i64> for Decoder<D>
where
    D: Digest,
{
    fn decode(&mut self, tag: Tag, input: &mut &[u8]) -> Result<i64, Error> {
        #[cfg(feature = "log")]
        begin!(self, "[{}]: sint64?", tag);

        self.peek().expect_header(input, tag, WireType::SInt64)?;
        self.peek().decode_sint64(input)
    }
}

impl<D> DecodeRef<[u8]> for Decoder<D>
where
    D: Digest,
{
    fn decode_ref<'a>(&mut self, tag: Tag, input: &mut &'a [u8]) -> Result<&'a [u8], Error> {
        #[cfg(feature = "log")]
        begin!(self, "[{}]: bytes?", tag);

        self.peek().expect_header(input, tag, WireType::Bytes)?;
        self.peek().decode_bytes(input)
    }
}

impl<D> DecodeRef<str> for Decoder<D>
where
    D: Digest,
{
    fn decode_ref<'a>(&mut self, tag: Tag, input: &mut &'a [u8]) -> Result<&'a str, Error> {
        #[cfg(feature = "log")]
        begin!(self, "[{}]: string?", tag);

        self.peek().expect_header(input, tag, WireType::String)?;
        self.peek().decode_string(input)
    }
}

impl<D, M> DecodeSeq<M, D> for Decoder<D>
where
    D: Digest,
    M: Message,
{
    fn decode_seq<'a>(
        &mut self,
        tag: Tag,
        input: &mut &'a [u8],
    ) -> Result<sequence::Iter<'a, M, D>, Error> {
        #[cfg(feature = "log")]
        begin!(self, "[{}]: seq<msg>?", tag);

        self.peek().expect_header(input, tag, WireType::Sequence)?;
        let seq_bytes = self.peek().decode_sequence(WireType::Message, input)?;
        let decoder = sequence::Decoder::new(WireType::Message, seq_bytes.len());

        Ok(sequence::Iter::new(decoder, seq_bytes))
    }
}

impl<D> DecodeSeq<u64, D> for Decoder<D>
where
    D: Digest,
{
    fn decode_seq<'a>(
        &mut self,
        tag: Tag,
        input: &mut &'a [u8],
    ) -> Result<sequence::Iter<'a, u64, D>, Error> {
        #[cfg(feature = "log")]
        begin!(self, "[{}]: seq<uint64>?", tag);

        self.peek().expect_header(input, tag, WireType::Sequence)?;
        let seq_bytes = self.peek().decode_sequence(WireType::UInt64, input)?;
        let decoder = sequence::Decoder::new(WireType::UInt64, seq_bytes.len());

        Ok(sequence::Iter::new(decoder, seq_bytes))
    }
}

impl<D> DecodeSeq<i64, D> for Decoder<D>
where
    D: Digest,
{
    fn decode_seq<'a>(
        &mut self,
        tag: Tag,
        input: &mut &'a [u8],
    ) -> Result<sequence::Iter<'a, i64, D>, Error> {
        #[cfg(feature = "log")]
        begin!(self, "[{}]: seq<sint64>?", tag);

        self.peek().expect_header(input, tag, WireType::Sequence)?;
        let seq_bytes = self.peek().decode_sequence(WireType::SInt64, input)?;
        let decoder = sequence::Decoder::new(WireType::SInt64, seq_bytes.len());

        Ok(sequence::Iter::new(decoder, seq_bytes))
    }
}

#[cfg(all(test, feature = "sha2"))]
mod tests {
    use super::{Decode, DecodeRef};
    use crate::Decoder;

    #[test]
    fn decode_uint64() {
        let input = [138, 10, 85];
        let mut input_ref = &input[..];

        let value: u64 = Decoder::new().decode(42, &mut input_ref).unwrap();
        assert_eq!(value, 42);
        assert!(input_ref.is_empty());
    }

    #[test]
    fn decode_sint64() {
        let input = [206, 10, 167];
        let mut input_ref = &input[..];

        let value: i64 = Decoder::new().decode(43, &mut input_ref).unwrap();
        assert_eq!(value, -42);
        assert!(input_ref.is_empty());
    }

    #[test]
    fn decode_bytes() {
        let input = [73, 11, 98, 121, 116, 101, 115];
        let mut input_ref = &input[..];

        let bytes: &[u8] = Decoder::new().decode_ref(2, &mut input_ref).unwrap();
        assert_eq!(bytes, &[98, 121, 116, 101, 115]);
        assert!(input_ref.is_empty());
    }

    #[test]
    fn decode_string() {
        let input = [139, 7, 98, 97, 122];
        let mut input_ref = &input[..];

        let string: &str = Decoder::new().decode_ref(4, &mut input_ref).unwrap();
        assert_eq!(string, "baz");
        assert!(input_ref.is_empty());
    }
}
