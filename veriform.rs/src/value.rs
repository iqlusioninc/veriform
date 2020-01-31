//! The Value enum: a loosely typed way of representing veriform messages.

#[cfg(not(feature = "std"))]
pub use alloc::btree_map::BTreeMap;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
#[cfg(feature = "std")]
pub use std::collections::HashMap;

/// Integer ID -> Value mapping with `hashDoS`-resistant `HashMap` on std
#[cfg(feature = "std")]
pub type Map = HashMap<u64, Value>;

/// Integer ID -> Value mapping, using BTreeMap without std since we don't have HashMap
// TODO: hashDoS resistance in no_std environments
#[cfg(not(feature = "std"))]
pub type Map = BTreeMap<u64, Value>;

/// Represents any value that can occur in a veriform message
#[derive(Debug, PartialEq)]
pub enum Value {
    /// Represents 8-bit clean binary data.
    Data(Vec<u8>),

    /// Represents an unsigned 64-bit integer.
    UInt(u64),

    /// Represents a (potentially nested) veriform message.
    Message(Map),
}
