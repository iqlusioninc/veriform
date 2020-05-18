//! Helper functions called from custom derive output

use crate::{
    decoder::sequence,
    decoder::{DecodeSeq, Decoder},
    encoder::Encoder,
    error::{self, Error},
    field::{Tag, WireType},
    message::{Element, Message},
};
use digest::Digest;
use heapless::ArrayLength;

/// Make sure input has been consumed
pub fn check_input_consumed(input: &[u8]) -> Result<(), Error> {
    if input.is_empty() {
        Ok(())
    } else {
        Err(error::Kind::TrailingData.into())
    }
}

/// Decode a sequence of messages
// TODO(tarcieri): support other sequence types
pub fn decode_message_seq<T, N, D>(
    decoder: &mut Decoder<D>,
    tag: Tag,
    input: &mut &[u8],
) -> Result<heapless::Vec<T, N>, Error>
where
    T: Message,
    N: ArrayLength<T>,
    D: Digest,
{
    let mut result = heapless::Vec::new();
    let seq_iter: sequence::Iter<'_, T, D> = decoder.decode_seq(tag, input)?;

    for elem in seq_iter {
        result.push(elem?).map_err(|_| error::Kind::Decode {
            element: Element::Value,
            wire_type: WireType::Sequence,
        })?
    }

    Ok(result)
}

/// Encode a sequence of messages
pub fn encode_message_seq<T>(
    encoder: &mut Encoder<'_>,
    tag: Tag,
    critical: bool,
    seq: &[T],
) -> Result<(), Error>
where
    T: Message,
{
    let body_len: usize = seq
        .iter()
        .map(|msg| {
            // compute length with additional length prefix
            let encoded_len = msg.encoded_len();
            vint64::encoded_len(encoded_len as u64)
                .checked_add(encoded_len)
                .unwrap()
        })
        .sum();

    encoder.message_seq(
        tag,
        critical,
        body_len,
        seq.iter().map(|elem| elem as &dyn Message),
    )
}

/// Unknown tag in enum
pub fn unknown_tag(tag: Tag) -> Error {
    error::Kind::FieldHeader {
        tag: Some(tag),
        wire_type: None,
    }
    .into()
}

/// Fallible version of the `Extend` trait used for consuming Veriform
/// sequences but with potential `max` limits (e.g. `heapless::Vec` size)
pub trait TryExtend<A> {
    /// Try to extend this type using the given iterator, returnin an error if
    /// capacity in the underlying buffer is exceeded
    fn try_extend<T>(&mut self, iter: T) -> Result<(), ()>
    where
        T: IntoIterator<Item = A>;
}

impl<T, N> TryExtend<T> for heapless::Vec<T, N>
where
    N: ArrayLength<T>,
{
    fn try_extend<I: IntoIterator<Item = T>>(&mut self, iter: I) -> Result<(), ()> {
        for elem in iter {
            self.push(elem).map_err(|_| ())?
        }

        Ok(())
    }
}

#[cfg(feature = "alloc")]
impl<T> TryExtend<T> for alloc::vec::Vec<T> {
    fn try_extend<I: IntoIterator<Item = T>>(&mut self, iter: I) -> Result<(), ()> {
        self.extend(iter);
        Ok(())
    }
}
