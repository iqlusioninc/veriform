//! zser message parser

#[cfg(not(feature = "std"))]
use collections::Vec;
use errors::*;
use varint;

/// Default maximum length of a zser message: 1kB
/// This is conservative as zser's main intended use case is a credential format
pub const DEFAULT_MAX_LENGTH: usize = 1024;

/// Default maximum depth (i.e. default max level of nested messages)
pub const DEFAULT_MAX_DEPTH: usize = 8;

/// Parser for zser messages
pub struct Parser<'m, H: Handler> {
    /// Maximum length message we'll accept
    max_length: usize,

    /// Maximum depth of nested messages allowed
    max_depth: usize,

    /// Bodies of nested messages remaining to be processed
    remaining: Vec<&'m [u8]>,

    /// Callbacks to invoke to construct the resulting type
    handler: H,
}

impl<'m, H: Handler> Parser<'m, H> {
    /// Create a new Parser
    pub fn new(handler: H) -> Parser<'m, H> {
        Parser {
            max_length: DEFAULT_MAX_LENGTH,
            max_depth: DEFAULT_MAX_DEPTH,
            remaining: Vec::new(),
            handler: handler,
        }
    }

    /// Parse the given zser message, invoking callbacks as necessary
    pub fn parse(&mut self, message: &'m [u8]) -> Result<()> {
        if message.len() > self.max_length {
            return Err(ErrorKind::OversizedMessage(message.len(), self.max_length).into());
        }

        if self.remaining.len() >= self.max_depth {
            return Err(ErrorKind::MaxDepthExceeded(self.max_depth).into());
        }

        self.remaining.push(message);

        while !self.remaining.last().expect("remaining").is_empty() {
            let (id, wiretype) = self.parse_field_prefix()?;

            match wiretype {
                0 => self.parse_u64(id)?,
                2 => self.parse_message(id)?,
                3 => self.parse_binary(id)?,
                _ => return Err(ErrorKind::UnknownWiretype(wiretype).into()),
            }
        }

        // In theory, this should never panic
        self.remaining.pop().expect("empty remaining stack");

        Ok(())
    }

    /// Finish parsing, returning the resulting object produced by the builder
    pub fn finish(self) -> Result<H::T> {
        if self.remaining.is_empty() {
            Ok(self.handler.finish())
        } else {
            Err(ErrorKind::UnconsumedMessages(self.remaining.len()).into())
        }
    }

    /// Pop the top item in the remaining stack and parse a varint from it
    pub fn parse_varint(&mut self) -> Result<(u64, &'m [u8])> {
        let slice = self.remaining.pop().expect("remaining slice");
        let mut slice_ref = &slice[..];
        let value = varint::decode(&mut slice_ref)?;

        Ok((value, slice_ref))
    }

    /// Parse the integer each field starts with, extracting field ID and wiretype
    pub fn parse_field_prefix(&mut self) -> Result<(u64, u64)> {
        let (value, remaining) = self.parse_varint()?;
        self.remaining.push(remaining);

        let field_id = value >> 3;
        let wiretype = value & 0x7;

        Ok((field_id, wiretype))
    }

    /// Parse a u64 value stored as a prefix varint
    pub fn parse_u64(&mut self, id: u64) -> Result<()> {
        let (value, remaining) = self.parse_varint()?;
        self.remaining.push(remaining);
        self.handler.uint64(id, value);

        Ok(())
    }

    /// Parse a blob of data that begins with a length prefix
    pub fn parse_length_prefixed_data(&mut self) -> Result<&'m [u8]> {
        let (length_u64, remaining) = self.parse_varint()?;
        let length = length_u64 as usize;

        if remaining.len() < length {
            let message = format!("want {} bytes, have {}", length, remaining.len());
            return Err(ErrorKind::TruncatedMessage(message).into());
        }

        let result = &remaining[..length];
        self.remaining.push(&remaining[length..]);

        Ok(result)
    }

    /// Parse a nested message
    pub fn parse_message(&mut self, id: u64) -> Result<()> {
        self.handler.begin_nested();

        let nested_message = self.parse_length_prefixed_data()?;
        self.parse(nested_message)?;
        self.handler.end_nested(id);

        Ok(())
    }

    /// Parse a field containing binary data
    pub fn parse_binary(&mut self, id: u64) -> Result<()> {
        let data = self.parse_length_prefixed_data()?;
        self.handler.binary(id, data);

        Ok(())
    }
}

/// Callback API used by the parser to process parsed data
// TODO: switch to serde
pub trait Handler {
    /// Type produced by this handler when parsing is complete
    type T;

    /// Called when a uint64 value with the given field ID is parsed
    fn uint64(&mut self, id: u64, value: u64);

    /// Called when we've received binary data with the given ID
    fn binary(&mut self, id: u64, data: &[u8]);

    /// Indicate we've entered a new nested message
    fn begin_nested(&mut self);

    /// Indicate we've reached the end of a nested message with the given ID
    fn end_nested(&mut self, id: u64);

    /// Return the fully parsed object
    fn finish(self) -> Self::T;
}
