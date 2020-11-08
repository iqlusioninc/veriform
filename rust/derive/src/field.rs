//! Support for the `#[field(...)] attribute

use darling::{FromField, FromVariant};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

/// Parsed `#[field(...)]` attribute.
///
/// When deriving the [`Message`] trait, every enum variant or field of a
/// struct MUST be tagged as a `field`.
///
/// # Example
///
/// ```ignore
/// #[derive(Message, Debug, Eq, PartialEq)]
/// pub struct ExampleMessageA {
///     #[field(tag = 0, wire_type = "uint64", critical = true)]
///     pub uint64_field: u64,
///
///     #[field(tag = 1, wire_type = "sint64")]
///     pub sint64_field: i64,
///
///     #[field(tag = 2, wire_type = "sequence", critical = true, max = 8)]
///     pub msg_sequence_field: Vec<ExampleMessageB>,
/// }
/// ```
///
/// [`Message`]: https://docs.rs/veriform/latest/veriform/derive.Message.html
#[derive(Debug, FromField, FromVariant)]
#[darling(attributes(field))]
pub(crate) struct Attrs {
    /// Tag which identifies the field
    tag: u64,

    /// Wire type of the field. See [`WireType`] for the available type names.
    wire_type: Ident,

    /// Is this field critical?
    #[darling(default)]
    critical: bool,

    /// Is this field optional?
    #[darling(default)]
    optional: bool,

    /// Size of a fixed-size field
    #[darling(default)]
    size: Option<usize>,

    /// Minimum size of a variable-sized field
    #[darling(default)]
    min: Option<usize>,

    /// Maximum size of a variable-sized field
    #[darling(default)]
    max: Option<usize>,
}

impl Attrs {
    /// Get the field identifier tag
    pub fn tag(&self) -> u64 {
        self.tag
    }

    /// Parse the wire type ident
    pub fn wire_type(&self) -> WireType {
        WireType::parse(self.wire_type.to_string())
    }

    /// Is this field critical?
    pub fn is_critical(&self) -> bool {
        self.critical
    }
}

/// Wire type identifiers for Veriform types
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum WireType {
    /// `bool`: boolean values - these are actually modeled as two different
    /// wire type identifiers (0 and 1) but consolidated for the purposes of
    /// this macro
    Bool,

    /// `uint64`: 64-bit unsigned integer
    UInt64,

    /// `sint64`: 64-bit (zigzag) signed integer
    SInt64,

    /// `bytes`: binary data
    Bytes,

    /// `string`: Unicode string
    String,

    /// `message`: nested Veriform message
    Message,

    /// `sequence`: sequences of other types (a.k.a. lists, arrays)
    Sequence,
}

impl WireType {
    /// Parse a wire type from a string
    ///
    /// Panics if the wire type is unrecognized
    pub fn parse(s: impl AsRef<str>) -> Self {
        match s.as_ref() {
            "bool" => WireType::Bool,
            "uint64" => WireType::UInt64,
            "sint64" => WireType::SInt64,
            "bytes" => WireType::Bytes,
            "string" => WireType::String,
            "message" => WireType::Message,
            "sequence" => WireType::Sequence,
            other => panic!("invalid wire type: {}", other),
        }
    }

    /// Get the Rust type for this token
    pub fn rust_type(self) -> Option<TokenStream> {
        let ty = match self {
            WireType::Bool => quote!(bool),
            WireType::UInt64 => quote!(u64),
            WireType::SInt64 => quote!(i64),
            WireType::Bytes => quote!(&[u8]),
            WireType::String => quote!(&str),
            _ => return None,
        };

        Some(ty)
    }

    /// Is the underlying Rust type to decode a reference type?
    pub fn is_ref_type(self) -> bool {
        self == WireType::Bytes || self == WireType::String
    }

    /// Is this [`WireType`] a `Message`?
    pub fn is_message(self) -> bool {
        match self {
            WireType::Message => true,
            _ => false,
        }
    }

    /// Is this [`WireType`] a `Sequence`?
    pub fn is_sequence(self) -> bool {
        match self {
            WireType::Sequence => true,
            _ => false,
        }
    }
}
