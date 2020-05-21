//! Trait providing helper methods for event handling shared between the
//! `message` and `sequence` decoders

use super::Event;
use crate::{
    error::{self, Error},
    field::WireType,
    message::Element,
    string,
};
use core::str;

/// Common functionality between the `message` and `sequence` decoders
pub trait Decodable {
    /// Process the given input data, advancing the slice for the amount of
    /// data processed, and returning any decoded events.
    fn decode<'a>(&mut self, input: &mut &'a [u8]) -> Result<Option<Event<'a>>, Error>;

    /// Decode a length delimited value, expecting the given wire type
    fn decode_dynamically_sized_value<'a>(
        &mut self,
        expected_type: WireType,
        input: &mut &'a [u8],
    ) -> Result<&'a [u8], Error>;

    /// Decode an expected `uint64`, returning an error for anything else
    fn decode_uint64(&mut self, input: &mut &[u8]) -> Result<u64, Error> {
        match self.decode(input)? {
            Some(Event::UInt64(value)) => Ok(value),
            _ => Err(error::Kind::Decode {
                element: Element::Value,
                wire_type: WireType::UInt64,
            }
            .into()),
        }
    }

    /// Decode an expected `sint64`, returning an error for anything else
    fn decode_sint64(&mut self, input: &mut &[u8]) -> Result<i64, Error> {
        match self.decode(input)? {
            Some(Event::SInt64(value)) => Ok(value),
            _ => Err(error::Kind::Decode {
                element: Element::Value,
                wire_type: WireType::SInt64,
            }
            .into()),
        }
    }

    /// Decode an expected `bytes` field, returning an error for anything else
    fn decode_bytes<'a>(&mut self, input: &mut &'a [u8]) -> Result<&'a [u8], Error> {
        self.decode_dynamically_sized_value(WireType::Bytes, input)
    }

    /// Decode an expected `string` field, returning an error for anything else
    fn decode_string<'a>(&mut self, input: &mut &'a [u8]) -> Result<&'a str, Error> {
        let bytes = self.decode_dynamically_sized_value(WireType::String, input)?;

        let s = str::from_utf8(bytes).map_err(|e| error::Kind::Utf8 {
            valid_up_to: e.valid_up_to(),
        })?;

        string::ensure_canonical(s)
    }

    /// Decode an expected `message` field, returning an error for anything else
    fn decode_message<'a>(&mut self, input: &mut &'a [u8]) -> Result<&'a [u8], Error> {
        self.decode_dynamically_sized_value(WireType::Message, input)
    }

    /// Decode an expected `sequence` field, returning an error for anything else
    fn decode_sequence<'a>(
        &mut self,
        expected_type: WireType,
        input: &mut &'a [u8],
    ) -> Result<&'a [u8], Error> {
        let length = match self.decode(input)? {
            Some(Event::SequenceHeader { wire_type, length }) if wire_type == expected_type => {
                length
            }
            _ => {
                return Err(error::Kind::Decode {
                    element: Element::SequenceHeader,
                    wire_type: expected_type,
                }
                .into())
            }
        };

        match self.decode(input)? {
            Some(Event::ValueChunk {
                bytes, remaining, ..
            }) => {
                if remaining == 0 {
                    debug_assert_eq!(length, bytes.len());
                    Ok(bytes)
                } else {
                    Err(error::Kind::Truncated {
                        remaining,
                        wire_type: WireType::Sequence,
                    }
                    .into())
                }
            }
            _ => Err(error::Kind::Decode {
                element: Element::Value,
                wire_type: WireType::Sequence,
            }
            .into()),
        }
    }
}
