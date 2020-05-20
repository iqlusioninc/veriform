//! Custom derive support for the `Message` trait

use crate::{
    digest,
    field::{self, WireType},
};
use darling::{FromField, FromVariant};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{DataEnum, DataStruct, Field, Ident};
use synstructure::Structure;

/// Custom derive for `Message`
pub(crate) fn derive(s: Structure<'_>) -> TokenStream {
    match &s.ast().data {
        syn::Data::Enum(data) => DeriveEnum::new().derive(s, data),
        syn::Data::Struct(data) => DeriveStruct::new().derive(s, data),
        other => panic!("can't derive Message on: {:?}", other),
    }
}

/// Derive `Message` on an enum
struct DeriveEnum {
    /// Body of `Message::decode()` in-progress for an enum
    decode_body: TokenStream,

    /// Body of `Message::encode()` in-progress for an enum
    encode_body: TokenStream,

    /// Body of `Message::encoded_len()` in-progress for an enum
    encoded_len_body: TokenStream,
}

impl DeriveEnum {
    /// Create a new [`DeriveStruct`]
    pub fn new() -> Self {
        Self {
            decode_body: TokenStream::new(),
            encode_body: TokenStream::new(),
            encoded_len_body: TokenStream::new(),
        }
    }

    /// Derive `Message` on an enum
    // TODO(tarcieri): hoist as much of this code out of the proc macro as possible
    fn derive(mut self, s: Structure<'_>, data: &DataEnum) -> TokenStream {
        if s.variants().len() != data.variants.len() {
            panic!(
                "unexpected number of variants ({} vs {})",
                s.variants().len(),
                data.variants.len()
            );
        }

        for (variant_info, variant) in s.variants().iter().zip(&data.variants) {
            let attrs = field::Attrs::from_variant(variant).unwrap_or_else(|e| {
                panic!("error parsing field attributes: {}", e);
            });

            self.derive_decode_match_arm(&variant.ident, &attrs);

            variant_info
                .each(|bi| encode_field(&bi.binding, &attrs))
                .to_tokens(&mut self.encode_body);

            variant_info
                .each(|bi| encoded_len_for_field(&bi.binding, &attrs))
                .to_tokens(&mut self.encoded_len_body)
        }

        let decode_body = self.decode_body;
        let encode_body = self.encode_body;
        let encoded_len_body = self.encoded_len_body;

        // TODO(tarcieri): ensure input is empty when message is finished decoding
        s.gen_impl(quote! {
            gen impl Message for @Self {
                fn decode<D>(
                    decoder: &mut veriform::decoder::Decoder<D>,
                    mut input: &[u8]
                ) -> Result<Self, veriform::Error>
                where
                    D: veriform::digest::Digest,
                {
                    #[allow(unused_imports)]
                    use veriform::decoder::Decodable;
                    #[allow(unused_imports)]
                    use core::convert::TryInto;

                    let msg = match decoder.peek().decode_header(&mut input)?.tag {
                        #decode_body
                        tag => return Err(veriform::derive_helpers::unknown_tag(tag))
                    };

                    veriform::derive_helpers::check_input_consumed(input)?;
                    Ok(msg)
                }

                fn encode<'a>(
                    &self,
                    buffer: &'a mut [u8]
                ) -> Result<&'a [u8], veriform::Error> {
                    let mut encoder = veriform::Encoder::new(buffer);

                    match self {
                        #encode_body
                    }

                    Ok(encoder.finish())
                }

                fn encoded_len(&self) -> usize {
                    match self {
                        #encoded_len_body
                    }
                }
            }
        })
    }

    /// Derive a match arm of an enum `decode` method
    fn derive_decode_match_arm(&mut self, name: &Ident, attrs: &field::Attrs) {
        let tag = attrs.tag();

        let decode_variant = match attrs.wire_type() {
            WireType::Bool => quote! {
                decoder
                    .peek()
                    .decode_bool(&mut input)?
                    .try_into()
                    .map(Self::#name)
                    .map_err(|_| veriform::field::WireType::True.decoding_error())?
            },
            WireType::UInt64 => quote! {
                decoder
                    .peek()
                    .decode_uint64(&mut input)?
                    .try_into()
                    .map(Self::#name)
                    .map_err(|_| veriform::field::WireType::UInt64.decoding_error())?
            },
            WireType::SInt64 => quote! {
                decoder
                    .peek()
                    .decode_sint64(&mut input)?
                    .try_into()
                    .map(Self::#name)
                    .map_err(|_| veriform::field::WireType::SInt64.decoding_error())?
            },
            WireType::Bytes => quote! {
                decoder
                    .peek()
                    .decode_bytes(&mut input)?
                    .try_into()
                    .map(Self::#name)
                    .map_err(|_| veriform::field::WireType::Bytes.decoding_error())?
            },
            WireType::String => quote! {
                decoder
                    .peek()
                    .decode_string(&mut input)?
                    .try_into()
                    .map(Self::#name)
                    .map_err(|_| veriform::field::WireType::String.decoding_error())?
            },
            WireType::Message => quote! {
                decoder
                    .peek()
                    .decode_message(&mut input)?
                    .try_into()
                    .and_then(|bytes| veriform::Message::decode(decoder, bytes))
                    .map(Self::#name)
                    .map_err(|_| veriform::field::WireType::Message.decoding_error())?
            },
            WireType::Sequence => todo!(),
        };

        let match_arm = quote! {
            #tag => { #decode_variant }
        };

        match_arm.to_tokens(&mut self.decode_body);
    }
}

/// Derive `Message` on a struct
struct DeriveStruct {
    /// Body of `Message::decode()` in-progress for a struct
    decode_body: TokenStream,

    /// Instantiation of the struct at the end of `Message::decode()`
    inst_body: TokenStream,

    /// Body of `Message::encode()` in-progress for a struct
    encode_body: TokenStream,

    /// Body of `Message::encoded_len()` in-progress for a struct
    encoded_len_body: TokenStream,
}

impl DeriveStruct {
    /// Create a new [`DeriveStruct`]
    pub fn new() -> Self {
        Self {
            decode_body: TokenStream::new(),
            inst_body: TokenStream::new(),
            encode_body: TokenStream::new(),
            encoded_len_body: quote!(0),
        }
    }

    /// Derive `Message` on a struct
    fn derive(mut self, s: Structure<'_>, data: &DataStruct) -> TokenStream {
        assert_eq!(s.variants().len(), 1);
        let variant = &s.variants()[0];
        let pattern = variant.pat();
        let bindings = &variant.bindings();

        if bindings.len() != data.fields.len() {
            panic!(
                "unexpected number of bindings ({} vs {})",
                bindings.len(),
                data.fields.len()
            );
        }

        for (binding_info, field) in bindings.iter().zip(&data.fields) {
            match parse_attr_ident(field).to_string().as_ref() {
                "field" => self.derive_field(field, &binding_info.binding),
                "digest" => self.derive_digest(field),
                other => panic!("unknown attribute: {}", other),
            }
        }

        let decode_body = self.decode_body;
        let inst_body = self.inst_body;
        let encode_body = self.encode_body;
        let encoded_len_body = self.encoded_len_body;

        s.gen_impl(quote! {
            gen impl Message for @Self {
                fn decode<D>(
                    decoder: &mut veriform::decoder::Decoder<D>,
                    mut input: &[u8]
                ) -> Result<Self, veriform::Error>
                where
                    D: veriform::digest::Digest,
                {
                    #[allow(unused_imports)]
                    use veriform::decoder::Decode;

                    #decode_body

                    Ok(Self { #inst_body })
                }

                fn encode<'a>(
                    &self,
                    buffer: &'a mut [u8]
                ) -> Result<&'a [u8], veriform::Error> {
                    let mut encoder = veriform::Encoder::new(buffer);

                    match self {
                        #pattern => { #encode_body }
                    }

                    Ok(encoder.finish())
                }

                fn encoded_len(&self) -> usize {
                    match self {
                        #pattern => { #encoded_len_body }
                    }
                }
            }
        })
    }

    /// Derive handling for a particular `#[field(...)]`
    fn derive_field(&mut self, field: &Field, binding: &Ident) {
        let name = parse_field_name(field);

        let attrs = field::Attrs::from_field(field).unwrap_or_else(|e| {
            panic!("error parsing field attributes: {}", e);
        });

        self.derive_decode_field(name, &attrs);

        let inst_field = quote!(#name,);
        inst_field.to_tokens(&mut self.inst_body);

        let enc_field = encode_field(binding, &attrs);
        let enc_field_with_semicolon = quote!(#enc_field;);
        enc_field_with_semicolon.to_tokens(&mut self.encode_body);

        let enc_field_len = encoded_len_for_field(binding, &attrs);
        let enc_field_len_with_plus = quote!(+ #enc_field_len);
        enc_field_len_with_plus.to_tokens(&mut self.encoded_len_body);
    }

    /// Derive a match arm of an struct `decode` method
    fn derive_decode_field(&mut self, name: &Ident, attrs: &field::Attrs) {
        let tag = attrs.tag();
        let wire_type = attrs.wire_type();

        match wire_type.rust_type() {
            Some(ty) => quote! {
                let #name: #ty = decoder.decode(#tag, &mut input)?;
            },
            None => {
                if wire_type.is_message() {
                    quote! {
                        let #name = decoder.decode(#tag, &mut input)?;
                    }
                } else if wire_type.is_sequence() {
                    // TODO(tarcieri): hoist more of this into a `derive_helper` function?
                    quote! {
                        let #name = veriform::derive_helpers::decode_message_seq(
                            decoder,
                            #tag,
                            &mut input
                        )?;
                    }
                } else {
                    unreachable!();
                }
            }
        }
        .to_tokens(&mut self.decode_body);
    }

    /// Derive handling for a `#[digest(...)]` member of a struct
    fn derive_digest(&mut self, field: &Field) {
        let name = parse_field_name(field);

        let attrs = digest::Attrs::from_field(field).unwrap_or_else(|e| {
            panic!("error parsing digest attributes: {}", e);
        });

        // TODO(tarcieri): support additional algorithms?
        match attrs.alg() {
            digest::Algorithm::Sha256 => self.derive_sha256_digest(&name),
        }
    }

    /// Derive computing a SHA-256 digest of a message
    fn derive_sha256_digest(&mut self, name: &Ident) {
        let fill_digest = quote! {
            let mut #name = veriform::Sha256Digest::default();
            decoder.fill_digest(&mut #name)?;
        };

        fill_digest.to_tokens(&mut self.decode_body);

        let inst_field = quote!(#name: Some(#name),);
        inst_field.to_tokens(&mut self.inst_body);
    }
}

impl Default for DeriveStruct {
    fn default() -> Self {
        Self::new()
    }
}

/// Parse the name of a field
fn parse_field_name(field: &Field) -> &Ident {
    field
        .ident
        .as_ref()
        .unwrap_or_else(|| panic!("no name on struct field (e.g. tuple structs unsupported)"))
}

/// Parse an attribute `Ident` i.e. `#[myattributeident(...)]`
fn parse_attr_ident(field: &Field) -> &Ident {
    assert_eq!(field.attrs.len(), 1);

    let attr = &field.attrs[0];
    let attr_segments = &attr.path.segments;

    assert_eq!(attr_segments.len(), 1);
    &attr_segments[0].ident
}

/// Encode a field of a message
fn encode_field(binding: &Ident, attrs: &field::Attrs) -> TokenStream {
    let tag = attrs.tag();
    let critical = attrs.is_critical();

    match attrs.wire_type() {
        WireType::Bool => todo!(),
        WireType::UInt64 => quote! { encoder.uint64(#tag, #critical, *#binding)? },
        WireType::SInt64 => quote! { encoder.sint64(#tag, #critical, *#binding)? },
        WireType::Bytes => quote! { encoder.bytes(#tag, #critical, #binding)? },
        WireType::String => quote! { encoder.string(#tag, #critical, #binding)? },
        WireType::Message => quote! { encoder.message(#tag, #critical, #binding)? },
        WireType::Sequence => quote! {
            // TODO(tarcieri): support other types of sequences besides messages
            veriform::derive_helpers::encode_message_seq(&mut encoder, #tag, #critical, #binding)?;
        },
    }
}

/// Compute the encoded length of a field
fn encoded_len_for_field(binding: &Ident, attrs: &field::Attrs) -> TokenStream {
    let tag = attrs.tag();

    match attrs.wire_type() {
        WireType::Bool => todo!(),
        WireType::UInt64 => quote! { veriform::field::length::uint64(#tag, *#binding) },
        WireType::SInt64 => quote! { veriform::field::length::sint64(#tag, *#binding) },
        WireType::Bytes => quote! { veriform::field::length::bytes(#tag, #binding) },
        WireType::String => quote! { veriform::field::length::string(#tag, #binding) },
        WireType::Message => quote! { veriform::field::length::message(#tag, #binding) },
        WireType::Sequence => quote! {
            // TODO(tarcieri): support other types of sequences besides messages
            veriform::field::length::message_seq(
                #tag,
                #binding.iter().map(|elem| elem as &dyn veriform::Message)
            )
        },
    }
}
