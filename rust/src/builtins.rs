//! Built-in message types: Veriform's "standard library".
//!
//! These are the equivalent of Protobufs' "well-known types"

#[cfg(feature = "tai64")]
mod timestamp;

#[cfg(feature = "uuid")]
mod uuid;

#[cfg(feature = "tai64")]
pub use self::timestamp::Timestamp;

#[cfg(feature = "uuid")]
pub use self::uuid::Uuid;
