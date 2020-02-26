//! Trait providing helper methods for event handling shared between the
//! `message` and `sequence` decoders

use super::Event;
use crate::{error::Error, field::WireType};
use core::str;

/// Common functionality between the `message` and `sequence` decoders
pub trait Decodable {
    /// Process the given input data, advancing the slice for the amount of
    /// data processed, and returning any decoded events.
    fn decode<'a>(&mut self, input: &mut &'a [u8]) -> Result<Option<Event<'a>>, Error>;

    /// Decode a length delimited value, expecting the given wire type
    fn decode_length_delimited_value<'a>(
        &mut self,
        input: &mut &'a [u8],
        expected_type: WireType,
    ) -> Result<&'a [u8], Error>;

    /// Decode an expected `uint64`, returning an error for anything else
    fn decode_uint64(&mut self, input: &mut &[u8]) -> Result<u64, Error> {
        match self.decode(input)? {
            Some(Event::UInt64(value)) => Ok(value),
            _ => Err(Error::Decode),
        }
    }

    /// Decode an expected `sint64`, returning an error for anything else
    fn decode_sint64(&mut self, input: &mut &[u8]) -> Result<i64, Error> {
        match self.decode(input)? {
            Some(Event::SInt64(value)) => Ok(value),
            _ => Err(Error::Decode),
        }
    }

    /// Decode an expected `bytes` field, returning an error for anything else
    fn decode_bytes<'a>(&mut self, input: &mut &'a [u8]) -> Result<&'a [u8], Error> {
        self.decode_length_delimited_value(input, WireType::Bytes)
    }

    /// Decode an expected `string` field, returning an error for anything else
    fn decode_string<'a>(&mut self, input: &mut &'a [u8]) -> Result<&'a str, Error> {
        let bytes = self.decode_length_delimited_value(input, WireType::String)?;
        str::from_utf8(bytes).map_err(|e| Error::Utf8 {
            valid_up_to: e.valid_up_to(),
        })
    }

    /// Decode an expected `message` field, returning an error for anything else
    fn decode_message<'a>(&mut self, input: &mut &'a [u8]) -> Result<&'a [u8], Error> {
        self.decode_length_delimited_value(input, WireType::Message)
    }

    /// Decode an expected `sequence` field, returning an error for anything else
    fn decode_sequence<'a>(&mut self, input: &mut &'a [u8]) -> Result<(WireType, &'a [u8]), Error> {
        if let Some(Event::SequenceHeader { wire_type, length }) = self.decode(input)? {
            if let Some(Event::ValueChunk {
                bytes, remaining, ..
            }) = self.decode(input)?
            {
                if remaining == 0 {
                    debug_assert_eq!(length, bytes.len());
                    return Ok((wire_type, bytes));
                }
            }
        }

        Err(Error::Decode)
    }
}
