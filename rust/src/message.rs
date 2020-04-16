//! Veriform messages

// Conceptually inspired by the `prost::Message` trait:
// <https://github.com/danburkert/prost/blob/master/src/message.rs>
//
// Copyright (c) 2017 Dan Burkert and released under the Apache 2.0 license.

use crate::{Decoder, Error};

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// Elements of a message (used for errors)
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Element {
    /// Length delimiters for dynamically sized fields
    LengthDelimiter,

    /// Headers of sequences
    SequenceHeader,

    /// Tags identify the types of fields
    Tag,

    /// Field values (i.e. inside the body of a field value)
    Value,
}

/// Veriform messages
pub trait Message {
    /// Decode a Veriform message contained in the provided slice using the
    /// given [`Decoder`].
    fn decode(decoder: &mut Decoder, input: &[u8]) -> Result<Self, Error>
    where
        Self: Sized;

    /// Encode this message as Veriform into the provided buffer, returning
    /// a slice containing the encoded message on success.
    fn encode<'a>(&self, buffer: &'a mut [u8]) -> Result<&'a [u8], Error>;

    /// Get the length of a message after being encoded as Veriform.
    fn encoded_len(&self) -> usize;

    /// Encode this message as Veriform, allocating returning a byte vector
    /// on success.
    #[cfg(feature = "alloc")]
    fn encode_vec(&self) -> Result<Vec<u8>, Error> {
        let mut encoded = vec![0; self.encoded_len()];
        self.encode(&mut encoded)?;
        Ok(encoded)
    }
}
