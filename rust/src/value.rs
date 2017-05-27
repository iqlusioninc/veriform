//! The Value enum: a loosely typed way of representing zser messages.

#[cfg(not(feature = "std"))]
pub use collections::HashMap;

#[cfg(feature = "std")]
pub use std::collections::HashMap;

/// Integer ID -> Value mapping used to represent messages internally
pub type Map = HashMap<u64, Value>;

/// Represents any value that can occur in a zser message
#[derive(Debug, PartialEq)]
pub enum Value {
    /// Represents 8-bit clean binary data.
    Data(Vec<u8>),

    /// Represents an unsigned 64-bit integer.
    UInt(u64),

    /// Represents a (potentially nested) zser message.
    Message(Map),
}
