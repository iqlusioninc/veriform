//! Custom derive support for the `Message` trait

use crate::field::{self, WireType};
use darling::{FromField, FromVariant};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{DataEnum, DataStruct, Ident};
use synstructure::Structure;

/// Custom derive for `Message`
pub(crate) fn derive(s: Structure<'_>) -> TokenStream {
    match &s.ast().data {
        syn::Data::Enum(data) => derive_enum(s, data),
        syn::Data::Struct(data) => derive_struct(s, data),
        other => panic!("can't derive Message on: {:?}", other),
    }
}

/// Derive `Message` on an enum
// TODO(tarcieri): hoist as much of this code out of the proc macro as possible
fn derive_enum(s: Structure<'_>, data: &DataEnum) -> TokenStream {
    let mut decode_body = TokenStream::new();
    let mut encode_body = TokenStream::new();
    let mut encoded_len_body = TokenStream::new();

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

        derive_enum_decode_match_arm(&variant.ident, &attrs).to_tokens(&mut decode_body);

        variant_info
            .each(|bi| encode_field(&bi.binding, &attrs))
            .to_tokens(&mut encode_body);

        variant_info
            .each(|bi| encoded_len_for_field(&bi.binding, &attrs))
            .to_tokens(&mut encoded_len_body)
    }

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
fn derive_enum_decode_match_arm(name: &Ident, attrs: &field::Attrs) -> TokenStream {
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

    quote! {
        #tag => { #decode_variant }
    }
}

/// Derive `Message` on a struct
fn derive_struct(s: Structure<'_>, data: &DataStruct) -> TokenStream {
    let mut decode_body = TokenStream::new();
    let mut inst_body = TokenStream::new();
    let mut encode_body = TokenStream::new();
    let mut encoded_len_body = quote!(0);

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
        let attrs = field::Attrs::from_field(field).unwrap_or_else(|e| {
            panic!("error parsing field attributes: {}", e);
        });

        let name = field.ident.as_ref().unwrap_or_else(|| {
            panic!("no name on struct field (e.g. tuple structs are unsupported)")
        });

        derive_struct_decode_match_arm(name, &attrs).to_tokens(&mut decode_body);

        let inst_field = quote!(#name,);
        inst_field.to_tokens(&mut inst_body);

        let enc_field = encode_field(&binding_info.binding, &attrs);
        let enc_field_with_semicolon = quote!(#enc_field;);
        enc_field_with_semicolon.to_tokens(&mut encode_body);

        let enc_field_len = encoded_len_for_field(&binding_info.binding, &attrs);
        let enc_field_len_with_plus = quote!(+ #enc_field_len);
        enc_field_len_with_plus.to_tokens(&mut encoded_len_body);
    }

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

/// Derive a match arm of an struct `decode` method
fn derive_struct_decode_match_arm(name: &Ident, attrs: &field::Attrs) -> TokenStream {
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
