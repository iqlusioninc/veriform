//! Message decoding traits.
//!
//! These traits infer the wire type to decode using the type system.
//!
//! They're intened to be impl'd by `veriform::decoder::Decoder`.

use super::sequence;
use crate::{field::Tag, Error};
use digest::Digest;

/// Try to decode a field to a value of the given type.
///
/// This trait is intended to be impl'd by the `Decoder` type.
pub trait Decode<T> {
    /// Try to decode a value of type `T`
    fn decode(&mut self, tag: Tag, input: &mut &[u8]) -> Result<T, Error>;
}

/// Try to decode a field to a reference of the given type.
///
/// This trait is intended to be impl'd by the `Decoder` type.
pub trait DecodeRef<T: ?Sized> {
    /// Try to decode a reference to type `T`
    fn decode_ref<'a>(&mut self, tag: Tag, input: &mut &'a [u8]) -> Result<&'a T, Error>;
}

/// Decode a sequence of values to a [`sequence::Iter`].
///
/// This trait is intended to be impl'd by the `Decoder` type.
pub trait DecodeSeq<T, D>
where
    D: Digest,
{
    /// Try to decode a sequence of values of type `T`
    fn decode_seq<'a>(
        &mut self,
        tag: Tag,
        input: &mut &'a [u8],
    ) -> Result<sequence::Iter<'a, T, D>, Error>;
}
