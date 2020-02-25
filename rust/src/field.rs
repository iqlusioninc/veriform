//! Fields (i.e. key/value pair) in a message

mod header;
pub mod length;
mod wire_type;

pub use self::{header::Header, wire_type::WireType};

/// Tag which identifies a field
pub type Tag = u64;
