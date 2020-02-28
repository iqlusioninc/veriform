//! Built-in message types: Veriform's "standard library".
//!
//! These are the equivalent of Protobufs' "well-known types"

#[cfg(feature = "tai64")]
mod tai64;

#[cfg(feature = "uuid")]
mod uuid;

#[cfg(feature = "tai64")]
pub use self::tai64::TAI64N;

#[cfg(feature = "uuid")]
pub use self::uuid::Uuid;
