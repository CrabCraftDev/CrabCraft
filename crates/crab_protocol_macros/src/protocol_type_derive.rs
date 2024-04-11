use crate::generate_field_code;
use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{parse2, Data, DeriveInput, Error, Result};

pub(crate) fn derive_protocol_type_impl(item: TokenStream) -> Result<TokenStream> {
    let input = parse2::<DeriveInput>(item)?;
    let input_name = input.ident.clone();
    let input_name_str = input_name.to_string();
    let input_span = input.span();
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let serialize_impl;
    let deserialize_impl;

    match input.data {
        Data::Enum(_data_enum) => {
            serialize_impl = quote! { buf.put_var_int(*self as i32); };
            deserialize_impl = quote! {
                let value = buf.get_var_int()? as usize;
                Self::from_repr(value).ok_or(::crab_protocol::error::ProtocolError::EnumOutOfBounds(#input_name_str.to_string(), value))
            };
        }
        Data::Struct(data_struct) => {
            let (fields_deserialize, fields_serialize) = generate_field_code(&Data::Struct(data_struct), &input_span)?;

            serialize_impl = quote! { #(#fields_serialize)* };
            deserialize_impl = quote! {
                Ok(Self {
                    #(#fields_deserialize,)*
                })
            };
        }
        _ => return Err(Error::new(input_span, "Unexpected type")),
    };

    Ok(quote! {
        impl #impl_generics ::crab_protocol::protocol_type::ProtocolType for #input_name #ty_generics
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
