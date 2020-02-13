//! Veriform encoder

use crate::{
    field::{Header, Tag, WireType},
    Error,
};

/// Veriform encoder
pub struct Encoder<'a> {
    /// Mutable buffer containing the message
    buffer: &'a mut [u8],

    /// Running total length of the message
    length: usize,
}

impl<'a> Encoder<'a> {
    /// Create a new [`Encoder`] which writes into the provided buffer
    pub fn new(buffer: &'a mut [u8]) -> Self {
        Self { buffer, length: 0 }
    }

    /// Write a field containing an unsigned 64-bit integer
    pub fn uint64(&mut self, tag: Tag, value: u64) -> Result<(), Error> {
        self.write_header(tag, WireType::UInt64)?;
        self.write(vint64::encode(value))?;
        Ok(())
    }

    /// Write a field containing a signed 64-bit integer
    pub fn sint64(&mut self, tag: Tag, value: i64) -> Result<(), Error> {
        self.write_header(tag, WireType::SInt64)?;
        self.write(vint64::encode_signed(value))?;
        Ok(())
    }

    /// Write a field containing bytes
    pub fn bytes(&mut self, tag: Tag, bytes: &[u8]) -> Result<(), Error> {
        self.write_header(tag, WireType::Bytes)?;
        self.write(vint64::encode(bytes.len() as u64))?;
        self.write(bytes)?;
        Ok(())
    }

    /// Write a nested message. Length must be known in advance.
    ///
    /// Calls the given function to actually write the message, passing the
    /// current buffer to the other function, and replacing it with the newly
    /// returned slice upon success.
    pub fn message<F>(&mut self, tag: Tag, length: usize, f: F) -> Result<(), Error>
    where
        F: FnOnce(&mut [u8]) -> Result<(), Error>,
    {
        self.write_header(tag, WireType::Message)?;
        self.write(vint64::encode(length as u64))?;

        // Ensure there's remaining space in the buffer
        if length > (self.buffer.len() - self.length) {
            return Err(Error::Length);
        }

        let new_length = self.length.checked_add(length).unwrap();
        f(&mut self.buffer[self.length..new_length])?;
        self.length = new_length;

        Ok(())
    }

    /// Finish constructing a message, returning a slice of the buffer
    /// containing the serialized message
    pub fn finish(self) -> &'a mut [u8] {
        &mut self.buffer[..self.length]
    }

    /// Write a field header to the underlying buffer
    fn write_header(&mut self, tag: Tag, wire_type: WireType) -> Result<(), Error> {
        self.write(Header { tag, wire_type }.encode())
    }

    /// Write the given bytes to the underlying buffer.
    ///
    /// Returns an error if the buffer has insufficient space.
    fn write(&mut self, bytes: impl AsRef<[u8]>) -> Result<(), Error> {
        let bytes = bytes.as_ref();

        // Ensure there's remaining space in the buffer
        if bytes.len() > (self.buffer.len() - self.length) {
            return Err(Error::Length);
        }

        let new_length = self.length.checked_add(bytes.len()).unwrap();
        self.buffer[self.length..new_length].copy_from_slice(bytes);
        self.length = new_length;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Encoder;
    use crate::{
        decoder::{Decoder, Event},
        field::WireType,
    };

    macro_rules! try_decode {
        ($decoder:expr, $input:expr, $event:path) => {
            match $decoder.decode($input).unwrap() {
                Some($event(event)) => event,
                other => panic!(
                    concat!("expected ", stringify!($event), ", got: {:?}"),
                    other
                ),
            }
        };
    }

    #[test]
    fn encode_then_decode() {
        let mut buffer = [0u8; 1024];
        let mut encoder = Encoder::new(&mut buffer);
        encoder.uint64(1, 42).unwrap();
        encoder.sint64(2, -1).unwrap();
        encoder.bytes(3, b"foobar").unwrap();
        let length = encoder.finish().len();
        let mut message = &buffer[..length];

        let mut decoder = Decoder::new();
        let header = try_decode!(decoder, &mut message, Event::FieldHeader);
        assert_eq!(header.tag, 1);
        assert_eq!(header.wire_type, WireType::UInt64);

        let value = try_decode!(decoder, &mut message, Event::UInt64);
        assert_eq!(value, 42);

        let header = try_decode!(decoder, &mut message, Event::FieldHeader);
        assert_eq!(header.tag, 2);
        assert_eq!(header.wire_type, WireType::SInt64);

        let value = try_decode!(decoder, &mut message, Event::SInt64);
        assert_eq!(value, -1);

        let header = try_decode!(decoder, &mut message, Event::FieldHeader);
        assert_eq!(header.tag, 3);
        assert_eq!(header.wire_type, WireType::Bytes);

        let msg_len = try_decode!(decoder, &mut message, Event::BytesLength);
        assert_eq!(msg_len, 6);

        match decoder.decode(&mut message).unwrap() {
            Some(Event::BytesChunk { bytes, remaining }) => {
                assert_eq!(remaining, 0);
                assert_eq!(bytes, b"foobar");
            }
            other => panic!(concat!("expected Event::BytesChunk, got: {:?}"), other),
        };

        assert_eq!(message.len(), 0);
    }
}
