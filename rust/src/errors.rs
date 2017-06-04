//! errors.rs: error types used by this crate


#[cfg(not(feature = "std"))]
use collections::boxed::Box;

#[cfg(not(feature = "std"))]
use collections::string::String;
use core::fmt::{self, Debug, Display};
use core::result;

#[cfg(feature = "std")]
use std::error;

/// Errors which can occur when serializing or deserializing zser messages
pub struct Error {
    err: Box<ErrorImpl>,
}

/// `Result` alias used by this crate
pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
struct ErrorImpl {
    kind: ErrorKind,
    byte_offset: Option<usize>,
}

/// Error variants encountered in this crate
#[derive(Debug)]
pub enum ErrorKind {
    /// Message exceeds configured size limit
    OversizedMessage(usize),

    /// Message exceeds configured maximum for nested messages
    MaxDepthExceeded(usize),

    /// Expected more data in message
    TruncatedMessage(String),

    /// Corruption detected in message
    CorruptedMessage(String),

    /// Message encoded with an unknown wiretype
    UnknownWiretype(u64),

    /// Unconsumed messages remaining in buffer
    UnconsumedMessages(usize),
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorKind::OversizedMessage(limit) => {
                write!(f, "message exceeds limit of {} bytes", limit)
            }
            ErrorKind::MaxDepthExceeded(max) => {
                write!(f, "max nested message depth of {} exceeded", max)
            }
            ErrorKind::TruncatedMessage(ref msg) => write!(f, "message truncated: {}", msg),
            ErrorKind::CorruptedMessage(ref msg) => write!(f, "message corrupted: {}", msg),
            ErrorKind::UnknownWiretype(wiretype) => write!(f, "unknown wiretype: {}", wiretype),
            ErrorKind::UnconsumedMessages(count) => {
                write!(f, "unconsumed messages in buffer: {} messages", count)
            }
        }
    }
}

#[cfg(feature = "std")]
impl error::Error for Error {
    fn description(&self) -> &str {
        match self.err.kind {
            ErrorKind::OversizedMessage(_) => "message too long",
            ErrorKind::MaxDepthExceeded(_) => "maximum number of nested messages exceeded",
            ErrorKind::TruncatedMessage(_) => "message truncated",
            ErrorKind::CorruptedMessage(_) => "message corrupted",
            ErrorKind::UnknownWiretype(_) => "unknown wiretype",
            ErrorKind::UnconsumedMessages(_) => "unconsumed messages in buffer",
        }
    }
}

impl From<ErrorKind> for Error {
    fn from(ek: ErrorKind) -> Self {
        Error {
            err: Box::new(ErrorImpl {
                              kind: ek,
                              byte_offset: None,
                          }),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&*self.err, f)
    }
}

impl Display for ErrorImpl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.byte_offset {
            Some(offset) => write!(f, "{} at byte {}", self.kind, offset),
            None => Display::fmt(&self.kind, f),
        }

    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(&*self.err, f)
    }
}
