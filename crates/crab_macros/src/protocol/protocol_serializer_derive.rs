use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::spanned::Spanned;
use syn::{parse2, Attribute, Data, DataStruct, DeriveInput, Error, Index, Result, Type};

use crate::protocol::args::{parse_attributes, ProtocolAttribute};

pub(crate) fn derive_protocol_serializer_impl(item: TokenStream) -> Result<TokenStream> {
    let input = parse2::<DeriveInput>(item)?;
    let input_name = input.ident.clone();
    let input_name_str = input_name.to_string();
    let input_span = input.span();
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

    let (serialize_impl, deserialize_impl) = match input.data {
        Data::Enum(_data_enum) => generate_enum(&input.attrs, &input_name_str),
        Data::Struct(data_struct) => generate_struct(&data_struct)?,
        _ => return Err(Error::new(input_span, "Unexpected type")),
    };

    Ok(quote! {
        impl #impl_generics ::crab_protocol::protocol_serializer::ProtocolSerializer for #input_name #type_generics
        #where_clause
        {
            fn serialize(&self, mut buf: &mut impl ::crab_protocol::buf::WriteByteBuf) -> Result<(), ::crab_protocol::error::ProtocolError> {
                #serialize_impl
                Ok(())
            }

            fn deserialize(buf: &mut impl ::crab_protocol::buf::ReadByteBuf) -> Result<Self, ::crab_protocol::error::ProtocolError> {
                #deserialize_impl
            }
        }
    })
}

fn generate_enum(input_attrs: &Vec<Attribute>, input_name_str: &str) -> (TokenStream, TokenStream) {
    let mut enum_type = quote! { i32 };
    let mut get_method = quote! { get_var_int()? as usize };
    let mut put_method = quote! { put_var_int };
    for attr in input_attrs {
        if !attr.path().is_ident("repr") {
            continue;
        }

        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("u8") {
                enum_type = quote! { u8 };
                get_method = quote! { get_u8() };
                put_method = quote! { put_u8 };
            }
            Ok(())
        })
        .expect("Unexpected error while parsing repr attribute");
    }

    let serialize_impl = quote! { buf.#put_method(*self as #enum_type); };
    let deserialize_impl = quote! {
        let value = buf.#get_method;
        Self::from_repr(value.into()).ok_or(::crab_protocol::error::ProtocolError::EnumOutOfBounds(#input_name_str.to_string(), value as usize))
    };
    (serialize_impl, deserialize_impl)
}

fn generate_struct(data: &DataStruct) -> Result<(TokenStream, TokenStream)> {
    let mut fields_deserialize = Vec::new();
    let mut fields_serialize = Vec::new();

    for (index, field) in data.fields.iter().enumerate() {
        let field_name = if let Some(ident) = &field.ident {
            ident.to_token_stream()
        } else {
            Index::from(index).to_token_stream()
        };

        let protocol_attribute = match parse_attributes(&field.attrs)? {
            Some(protocol_attribute) => protocol_attribute,
            None => {
                fields_deserialize.push(quote! {
                    #field_name: ::crab_protocol::protocol_serializer::ProtocolSerializer::deserialize(buf)?
                });
                fields_serialize.push(quote! {
                    ::crab_protocol::protocol_serializer::ProtocolSerializer::serialize(&self.#field_name, &mut buf)?;
                });
                continue;
            }
        };

        match protocol_attribute {
            ProtocolAttribute::Bounded(max_value) => {
                fields_deserialize.push(quote! {
                    #field_name: {
                        let field = ::crab_protocol::protocol_serializer::ProtocolSerializer::deserialize(buf)?;
                        ::crab_protocol::bounded::Bounded::check(&field, #max_value)?;
                        field
                    }
                });
                fields_serialize.push(quote! {
                    ::crab_protocol::bounded::Bounded::check(&self.#field_name, #max_value)?;
                    ::crab_protocol::protocol_serializer::ProtocolSerializer::serialize(&self.#field_name, &mut buf)?;
                });
            }
            ProtocolAttribute::Type(protocol_serializer) => {
                let protocol_serializer = protocol_serializer.unwrap();

                let conversion_method = if let Type::Path(type_path) = &protocol_serializer {
                    let first_type_name = &type_path.path.segments.first().unwrap().ident;
                    match first_type_name.to_string().as_str() {
                        "Option" => quote! { map(Into::into) },
                        "Vec" => quote! { iter().map(|&value| value.into()).collect() },
                        _ => quote! { into() },
                    }
                } else {
                    quote! { into() }
                };

                fields_deserialize.push(quote! {
                    #field_name: {
                        let field: #protocol_serializer = ::crab_protocol::protocol_serializer::ProtocolSerializer::deserialize(buf)?;
                        field.#conversion_method
                    }
                });
                fields_serialize.push(quote! {
                    let field: #protocol_serializer = self.#field_name.#conversion_method;
                    ::crab_protocol::protocol_serializer::ProtocolSerializer::serialize(&field, &mut buf)?;
                });
            }
        }
    }

    let serialize_impl = quote! { #(#fields_serialize)* };
    let deserialize_impl = quote! {
        Ok(Self {
            #(#fields_deserialize,)*
        })
    };

    Ok((serialize_impl, deserialize_impl))
}
