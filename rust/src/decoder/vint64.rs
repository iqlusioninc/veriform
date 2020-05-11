//! Decoder for `vint64` values

pub(crate) use vint64::signed::zigzag;

use crate::error::{self, Error};

/// Decoder for `vint64` values
#[derive(Clone, Debug, Default)]
pub struct Decoder {
    /// Length of the field header `vint64` (if known)
    length: Option<usize>,

    /// Position we are at reading in the input buffer
    pos: usize,

    /// Incoming data buffer
    buffer: [u8; 9],
}

impl Decoder {
    /// Create a new [`Decoder`]
    pub fn new() -> Self {
        Self::default()
    }

    /// Decode a `vint64` from the incoming data
    pub fn decode(&mut self, input: &mut &[u8]) -> Result<Option<u64>, Error> {
        if let Some(length) = self.length {
            self.fill_buffer(length, input);
            return self.maybe_decode(length);
        }

        if let Some(&first_byte) = input.first() {
            self.length = Some(vint64::decoded_len(first_byte));
            self.decode(input)
        } else {
            Ok(None)
        }
    }

    /// Fill the internal buffer with data, returning a [`FieldHeader`] if we're complete
    fn fill_buffer(&mut self, length: usize, input: &mut &[u8]) {
        let remaining = length.checked_sub(self.pos).unwrap();

        if input.len() < remaining {
            let new_pos = self.pos.checked_add(input.len()).unwrap();
            self.buffer[self.pos..new_pos].copy_from_slice(*input);
            self.pos = new_pos;
            *input = &[];
        } else {
            self.buffer[self.pos..length].copy_from_slice(&input[..remaining]);
            self.pos += remaining;
            *input = &input[remaining..];
        }
    }

    /// Attempt to decode the internal buffer if we've read its full contents
    fn maybe_decode(&self, length: usize) -> Result<Option<u64>, Error> {
        if self.pos < length {
            return Ok(None);
        }

        let mut buffer = &self.buffer[..length];
        vint64::decode(&mut buffer)
            .map(Some)
            .map_err(|_| error::Kind::VInt64.into())
    }
}
