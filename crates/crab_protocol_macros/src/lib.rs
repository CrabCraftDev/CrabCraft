mod args;
mod packet_derive;
mod protocol_type_derive;

use crate::args::{parse_attributes, ProtocolAttribute};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{Data, Error};

/// `ProtocolType` is a derive macro that helps in creating common Protocol Types like `VarInt Enum`, `Struct`, etc.
/// This macro simplifies the process of implementing the `ProtocolType` trait for a type.
///
/// The `ProtocolType` trait provides methods for serializing and deserializing the type into/from a byte buffer.
/// The `derive_protocol_type` macro generates an implementation of the `ProtocolType` trait for the type.
///
/// # Examples
///
/// Here is an example of how to use the `ProtocolType` derive macro with a struct:
///
/// ```ignore
/// #[derive(ProtocolType, Copy, Clone)]
/// pub struct MyStruct {
///     pub field1: i32,
///     pub field2: String,
/// }
/// ```
///
/// Here is an example of how to use the `ProtocolType` derive macro with an enum:
///
/// ```ignore
/// // `FromRepr` is required for enums!
/// #[derive(ProtocolType, FromRepr, Copy, Clone)]
/// pub enum MyEnum {
///     Variant1 = 1,
///     Variant2 = 2,
/// }
/// ```
///
///
/// Note: The `ProtocolType` derive macro can only be used with structs and enums.
#[proc_macro_derive(ProtocolType, attributes(protocol_attribute))]
pub fn derive_protocol_type(item: TokenStream) -> TokenStream {
    match protocol_type_derive::derive_protocol_type_impl(item.into()) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.into_compile_error().into(),
    }
}

/// Procedural macro that is used to derive the `Packet` trait for a struct.
///
/// The `Packet` trait provides methods for serializing and deserializing the struct into/from a byte buffer.
/// The `derive_packet` macro generates an implementation of the `Packet` trait for the struct.
///
/// The `Packet` trait requires three associated constants: `ID`, `SIDE`, and `STATE`.
/// These constants are provided through the `#[packet(id = "...", state = "...")]` attribute on the struct.
/// The `SIDE` constant is determined based on the name of the struct (whether it contains "S2C" or "C2S").
///
/// # Attributes
///
/// * `#[packet(id = "...", state = "...")]`: This attribute is required on the struct.
///   The `id` argument should be an integer representing the packet ID.
///   The `state` argument should be an expression that evaluates to a `ProtocolState` variant.
///
/// * `#[protocol_attribute(protocol_type = "...")]`: This attribute is optional and can be used on struct fields.
/// The `protocol_type` argument should be the name of a type that implements the `ProtocolType` trait.
/// If this attribute is not provided, the default implementation of `ProtocolType` will be used.
///
/// * `#[protocol_attribute(bounded = "...")]`: This attribute is optional and can be used on struct fields.
/// The `bounded` argument should be an integer representing the maximum value allowed for the field.
/// If the field value exceeds this maximum value, an error will be returned during serialization/deserialization.
///
/// # Examples
///
/// ```ignore
/// use crab_protocol::types::varint::VarInt;
/// use crab_protocol_macros::Packet;
///
/// #[derive(Packet, Debug, Clone)]
/// #[packet(id = 0, state = Handshake)]
/// pub struct HandshakeC2SPacket {
///     pub protocol_version: VarInt,
///     pub server_address: String,
///     pub server_port: u16,
///     pub next_state: VarInt,
/// }
/// ```
#[proc_macro_derive(Packet, attributes(packet, protocol_attribute))]
pub fn derive_packet(item: TokenStream) -> TokenStream {
    match packet_derive::derive_packet_impl(item.into()) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.into_compile_error().into(),
    }
}

pub(crate) fn generate_field_code(
    data: &Data,
    span: &Span,
) -> syn::Result<(Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>)> {
    let mut fields_deserialize = Vec::new();
    let mut fields_serialize = Vec::new();

    let data_struct = match data {
        Data::Struct(data_struct) => data_struct,
        _ => return Err(Error::new(*span, "Can be derived only for structs")),
    };

    let fields = &data_struct.fields;
    for field in fields.iter() {
        let field_name = field.ident.as_ref().expect("Unnamed field in struct");

        let protocol_attribute = match parse_attributes(&field.attrs)? {
            Some(protocol_attribute) => protocol_attribute,
            None => {
                fields_deserialize.push(quote! {
                    #field_name: ::crab_protocol::protocol_type::ProtocolType::deserialize(buf)?
                });
                fields_serialize.push(quote! {
                    ::crab_protocol::protocol_type::ProtocolType::serialize(&self.#field_name, &mut buf)?;
                });
                continue;
            }
        };

        match protocol_attribute {
            ProtocolAttribute::Bounded(max_value) => {
                fields_deserialize.push(quote! {
                    #field_name: {
                        let #field_name = ::crab_protocol::protocol_type::ProtocolType::deserialize(buf)?;
                        ::crab_protocol::bounded::Bounded::check(&#field_name, #max_value)?;
                        #field_name
                    }
                });
                fields_serialize.push(quote! {
                    ::crab_protocol::bounded::Bounded::check(&self.#field_name, #max_value)?;
                    ::crab_protocol::protocol_type::ProtocolType::serialize(&self.#field_name, &mut buf)?;
                });
            }
            ProtocolAttribute::Type(protocol_type) => {
                fields_deserialize.push(quote! {
                    #field_name: {
                        let #field_name: #protocol_type = ::crab_protocol::protocol_type::ProtocolType::deserialize(buf)?;
                        #field_name.into()
                    }
                });
                fields_serialize.push(quote! {
                    let #field_name: #protocol_type = self.#field_name.into();
                    ::crab_protocol::protocol_type::ProtocolType::serialize(&#field_name, &mut buf)?;
                });
            }
        }
    }

    Ok((fields_deserialize, fields_serialize))
}

extern crate proc_macro;
