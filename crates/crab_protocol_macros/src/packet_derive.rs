use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse2, DeriveInput, Error, Expr, Result};

use crate::args::parse_packet_args;

pub(crate) fn derive_packet_impl(item: TokenStream) -> Result<TokenStream> {
    let input = parse2::<DeriveInput>(item)?;
    let packet_args = parse_packet_args(&input.attrs)?.expect("Packet attribute missing!");
    let name = input.ident;

    let packet_id: Expr = packet_args.id.unwrap();
    let side = determine_side(&name.to_string(), &packet_args.span)?;
    let state = packet_args
        .state
        .ok_or_else(|| Error::new(packet_args.span, "State not provided for packet"))?;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics ::crab_protocol::packet::Packet for #name #ty_generics
        #where_clause
        {
            const ID: i32 = #packet_id;
            const SIDE: ::crab_protocol::protocol::PacketDirection = #side;
            const STATE: ::crab_protocol::protocol::ProtocolState = #state;
        }
    })
}

fn determine_side(name: &str, span: &Span) -> Result<TokenStream> {
    if name.contains("S2C") {
        Ok(quote!(::crab_protocol::protocol::PacketDirection::Clientbound))
    } else if name.contains("C2S") {
        Ok(quote!(::crab_protocol::protocol::PacketDirection::Serverbound))
    } else {
        Err(Error::new(*span, "Packet name doesn't contain S2C or C2S"))
    }
}
