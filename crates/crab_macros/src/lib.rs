extern crate proc_macro;

use crate::config::config_derive::config_macro;
use proc_macro::TokenStream;

mod core;
mod config;
mod protocol;
mod world;

/// `ProtocolSerializer` is a derive macro that helps in creating common Protocol Types like `Enum`, `Struct`, etc.
///
/// This macro simplifies the process of implementing the `ProtocolSerializer` trait for a type.
///
/// The `ProtocolSerializer` trait provides methods for serializing and deserializing the type into/from a byte buffer.
/// The `derive_protocol_serializer` macro generates an implementation of the `ProtocolSerializer` trait for the type.
///
/// # Attributes
///
/// * `#[packet(id = "...", state = "...")]`: This attribute is required on the struct.
///   The `id` argument should be an integer representing the packet ID.
///   The `state` argument should be an expression that evaluates to a `ProtocolState` variant.
///
/// * `#[protocol_attribute(protocol_type = "...")]`: This attribute is optional and can be used on struct fields.
///   The `protocol_type` argument should be the name of a type that implements the `ProtocolSerializer` trait.
///   If this attribute is not provided, the default implementation of `ProtocolSerializer` will be used.
///
/// * `#[protocol_attribute(bounded = "...")]`: This attribute is optional and can be used on struct fields.
///   The `bounded` argument should be an integer representing the maximum value allowed for the field.
///   If the field value exceeds this maximum value, an error will be returned during serialization/deserialization.
///
/// # Struct
///
/// Here is an example of how to use the `ProtocolSerializer` derive macro with a struct:
///
/// ```ignore
/// #[derive(ProtocolSerializer, Copy, Clone)]
/// pub struct MyStruct {
///     pub field1: i32,
///     pub field2: String,
/// }
/// ```
///
/// # Enum
///
/// Currently only `Byte` and `VarInt` are supported as the underlying type for enums.
/// You can define enum as `Byte` using `#[repr(u8)]`.
///
/// Here is an example of how to use the `ProtocolSerializer` derive macro with an enum:
///
/// ```ignore
/// // `FromRepr` is required for enums!
/// #[derive(ProtocolSerializer, FromRepr, Copy, Clone)]
/// pub enum MyEnum {
///     Variant1 = 1,
///     Variant2 = 2,
/// }
/// ```
///
/// Note: The `ProtocolSerializer` derive macro can only be used with structs and enums.
#[proc_macro_derive(ProtocolSerializer, attributes(protocol_attribute))]
pub fn derive_protocol_serializer(item: TokenStream) -> TokenStream {
    match protocol::protocol_serializer_derive::derive_protocol_serializer_impl(item.into()) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.into_compile_error().into(),
    }
}

/// Procedural macro that is used to derive the `Packet` trait for a struct.
///
/// The `Packet` trait requires three associated constants: `ID`, `SIDE`, and `STATE`.
/// These constants are provided through the `#[packet(id = "...", state = "...")]` attribute on the struct.
/// The `SIDE` constant is determined based on the name of the struct (whether it contains "S2C" or "C2S").
///
/// The `Packet` trait requires method for serialization and deserialization provided by `ProtocolSerializer`.
/// So it is recommended to use the `ProtocolSerializer` derive macro for the struct or implement `ProtocolSerializer` trait.
///
/// # Examples
///
/// ```ignore
/// use crab_protocol::types::varint::VarInt;
/// use crab_macros::Packet;
///
/// #[derive(Packet, ProtocolSerializer, Debug, Clone)]
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
    match protocol::packet_derive::derive_packet_impl(item.into()) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.into_compile_error().into(),
    }
}

/// Procedural macro that derives the `BlockData` trait for a struct.
///
/// The `BlockData` trait is used to define properties and behaviors of blocks in the world.
/// This macro simplifies the process of implementing the `BlockData` trait for a type.
///
/// # Examples
///
/// ```ignore
/// #[derive(BlockData)]
/// pub struct MyBlockData {
///     pub boolean: bool,
///     pub value: u16,
/// }
/// ```
#[proc_macro_derive(BlockData)]
pub fn derive_block_property(item: TokenStream) -> TokenStream {
    match world::block_property_derive::derive_block_property_impl(item.into()) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.into_compile_error().into(),
    }
}

#[proc_macro_derive(Config, attributes(default_value))]
pub fn config_derive(input: TokenStream) -> TokenStream {
    config_macro(input)
}


/// Derives an implementation of `TryFrom<AsyncEvent>` for the annotated type.
///
/// This macro allows converting an `AsyncEvent` into the annotated struct or enum,
/// provided the `AsyncEvent` contains a matching variant.
///
/// The matching variant is determined by stripping the `Event` suffix from the annotated type's name.
///
/// # Example
/// ```ignore
/// use crab_macros::AsyncEvent;
///
/// #[derive(AsyncEvent)]
/// struct MyEvent;
/// ```
#[proc_macro_derive(AsyncEvent)]
pub fn async_event(item: TokenStream) -> TokenStream {
    match core::async_event::async_event_impl(item.into()) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.into_compile_error().into(),
    }
}
