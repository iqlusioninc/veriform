//! Veriform encoder

use crate::{
    error::{self, Error},
    field::{Header, Tag, WireType},
    message::Message,
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
    pub fn uint64(&mut self, tag: Tag, critical: bool, value: u64) -> Result<(), Error> {
        self.write_header(tag, critical, WireType::UInt64)?;
        self.write(vint64::encode(value))
    }

    /// Write a field containing a signed 64-bit integer
    pub fn sint64(&mut self, tag: Tag, critical: bool, value: i64) -> Result<(), Error> {
        self.write_header(tag, critical, WireType::SInt64)?;
        self.write(vint64::signed::encode(value))
    }

    /// Write a message (nested inside of a field)
    pub fn message(
        &mut self,
        tag: Tag,
        critical: bool,
        message: &dyn Message,
    ) -> Result<(), Error> {
        let encoded_len = message.encoded_len();

        self.write_header(tag, critical, WireType::Message)?;
        self.write(vint64::encode(encoded_len as u64))?;

        // Ensure there's remaining space in the buffer
        if encoded_len > (self.buffer.len() - self.length) {
            return Err(error::Kind::Length.into());
        }

        let new_length = self.length.checked_add(encoded_len).unwrap();
        message.encode(&mut self.buffer[self.length..new_length])?;
        self.length = new_length;

        Ok(())
    }

    /// Write a sequence of messages (nested inside of a field)
    pub fn message_seq<'m>(
        &mut self,
        tag: Tag,
        critical: bool,
        length: usize,
        messages: impl Iterator<Item = &'m dyn Message>,
    ) -> Result<(), Error> {
        self.write_header(tag, critical, WireType::Sequence)?;

        // sequence header (type + length)
        self.write(vint64::encode(
            (length as u64) << 4 | WireType::Message as u64,
        ))?;

        let orig_length = self.length;

        for message in messages {
            let encoded_len = message.encoded_len();
            self.write(vint64::encode(encoded_len as u64))?;

            // Ensure there's remaining space in the buffer
            if encoded_len > self.buffer.len().checked_sub(self.length).unwrap() {
                return Err(error::Kind::Length.into());
            }

            let new_length = self.length.checked_add(encoded_len).unwrap();
            message.encode(&mut self.buffer[self.length..new_length])?;
            self.length = new_length;
        }

        // Ensure we wrote the expected number of bytes
        debug_assert_eq!(length, self.length.checked_sub(orig_length).unwrap());
        Ok(())
    }

    /// Write a field containing bytes
    pub fn bytes(&mut self, tag: Tag, critical: bool, bytes: &[u8]) -> Result<(), Error> {
        self.write_header(tag, critical, WireType::Bytes)?;
        self.write_value(bytes)
    }

    /// Write a field containing a string
    pub fn string(&mut self, tag: Tag, critical: bool, string: &str) -> Result<(), Error> {
        self.write_header(tag, critical, WireType::String)?;
        self.write_value(string.as_bytes())
    }

    /// Finish constructing a message, returning a slice of the buffer
    /// containing the serialized message
    pub fn finish(self) -> &'a [u8] {
        &self.buffer[..self.length]
    }

    /// Write a field header to the underlying buffer
    fn write_header(&mut self, tag: Tag, critical: bool, wire_type: WireType) -> Result<(), Error> {
        self.write(Header::new(tag, critical, wire_type).encode())
    }

    /// Write a dynamically sized value to the underlying buffer
    fn write_value(&mut self, bytes: &[u8]) -> Result<(), Error> {
        self.write(vint64::encode(bytes.len() as u64))?;
        self.write(bytes)
    }

    /// Write the given bytes to the underlying buffer.
    ///
    /// Returns an error if the buffer has insufficient space.
    fn write(&mut self, bytes: impl AsRef<[u8]>) -> Result<(), Error> {
        let bytes = bytes.as_ref();

        // Ensure there's remaining space in the buffer
        if bytes.len() > (self.buffer.len() - self.length) {
            return Err(error::Kind::Length.into());
        }

        let new_length = self.length.checked_add(bytes.len()).unwrap();
        self.buffer[self.length..new_length].copy_from_slice(bytes);
        self.length = new_length;

        Ok(())
    }
}

#[cfg(all(test, feature = "sha2"))]
mod tests {
    use super::Encoder;
    use crate::{decoder::Decodable, field::WireType};

    // TODO(tarcieri): rewrite tests with `crate::Decoder`
    type Decoder = crate::decoder::message::Decoder<sha2::Sha256>;

    const EXAMPLE_BYTES: &[u8] = b"foobar";
    const EXAMPLE_STRING: &str = "baz";

    #[test]
    fn encode_then_decode() {
        let mut buffer = [0u8; 1024];
        let mut encoder = Encoder::new(&mut buffer);

        encoder.uint64(1, false, 42).unwrap();
        encoder.sint64(2, false, -1).unwrap();
        encoder.bytes(3, false, EXAMPLE_BYTES).unwrap();
        encoder.string(4, false, EXAMPLE_STRING).unwrap();

        let length = encoder.finish().len();
        let mut message = &buffer[..length];

        let mut decoder = Decoder::new();
        let header = decoder.decode_header(&mut message).unwrap();
        assert_eq!(header.tag, 1);
        assert_eq!(header.wire_type, WireType::UInt64);

        let value = decoder.decode_uint64(&mut message).unwrap();
        assert_eq!(value, 42);

        let header = decoder.decode_header(&mut message).unwrap();
        assert_eq!(header.tag, 2);
        assert_eq!(header.wire_type, WireType::SInt64);

        let value = decoder.decode_sint64(&mut message).unwrap();
        assert_eq!(value, -1);

        let header = decoder.decode_header(&mut message).unwrap();
        assert_eq!(header.tag, 3);
        assert_eq!(header.wire_type, WireType::Bytes);

        let bytes = decoder.decode_bytes(&mut message).unwrap();
        assert_eq!(bytes, EXAMPLE_BYTES);

        let header = decoder.decode_header(&mut message).unwrap();
        assert_eq!(header.tag, 4);
        assert_eq!(header.wire_type, WireType::String);

        let string = decoder.decode_string(&mut message).unwrap();
        assert_eq!(string, EXAMPLE_STRING);

        assert!(message.is_empty());
    }
}
