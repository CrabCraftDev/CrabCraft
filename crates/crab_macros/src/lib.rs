extern crate proc_macro;

use crate::config::config_derive::config_macro;
use proc_macro::TokenStream;

mod config;
mod core;
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

/// Derive macro to implement the `Packet` trait for a struct.
/// This macro allows you to attach packet metadata using the `#[packet(...)]` attribute.
///
/// # Attributes
/// - `name`: A string literal representing the packet's name (used in the Minecraft codebase).
///
/// For packets used only in one protocol state, you can use:
/// - `id`: An integer literal representing the packet ID for a specific protocol state.
/// - `state`: An expression evaluating to a `ProtocolState` variant.
///
/// For packets that have different IDs depending on the protocol state, you can use:
/// - `ids(...)`: Specify multiple `state = id` pairs.
///
/// # Serialization
/// The `Packet` trait requires methods for serialization and deserialization.
/// It is recommended to also derive or implement `ProtocolSerializer` for your struct.
///
/// # Examples
///
/// Single-state packet:
/// ```ignore
/// use crab_protocol::types::varint::VarInt;
/// use crab_macros::Packet;
///
/// #[derive(Packet, ProtocolSerializer, Debug, Clone)]
/// #[packet(name = "intention", id = 0x00, state = Handshake)]
/// pub struct HandshakeC2SPacket {
///     pub protocol_version: VarInt,
///     pub server_address: String,
///     pub server_port: u16,
///     pub next_state: VarInt,
/// }
/// ```
///
/// Multi-state packet:
/// ```ignore
/// use crab_macros::Packet;
///
/// #[derive(Packet, ProtocolSerializer, Debug, Clone)]
/// #[packet(name = "ping_request", ids(Play = 0x25, Status = 0x01))]
/// pub struct PingRequestC2SPacket {
///     pub id: i64,
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

/// Procedural macro that derives the `Config` trait for a struct.
///
/// This macro generates implementations for configuration management,
/// allowing structs to be used as configuration entries with optional default values.
///
/// # Attributes
///
/// * `#[default_value = "..."]`: Specifies a default value for a field when the configuration is created.
///
/// # Example
///
/// ```ignore
/// #[derive(Config)]
/// pub struct ServerConfig {
///     #[default_value = 25565]
///     pub port: u16,
///     /// Comment
///     #[default_value = "127.0.0.1"]
///     pub host: String,
/// }
/// ```
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
#[proc_macro_derive(DetailedFrom, attributes(detailed))]
pub fn detailed_from(item: TokenStream) -> TokenStream {
    match core::detailed_from::detailed_from_impl(item.into()) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.into_compile_error().into(),
    }
}
