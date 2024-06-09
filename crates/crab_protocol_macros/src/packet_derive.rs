use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, DeriveInput, Error, Expr, Result};

use crate::args::parse_packet_args;

pub(crate) fn derive_packet_impl(item: TokenStream) -> Result<TokenStream> {
    let input = parse2::<DeriveInput>(item)?;
    let packet_args = parse_packet_args(&input.attrs)?.expect("Packet attribute missing!");
    let name = input.ident;

    let packet_id: Expr = packet_args.id.unwrap();
    let state = packet_args
        .state
        .ok_or_else(|| Error::new(packet_args.span, "State not provided for packet"))?;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics ::crab_protocol::packet::Packet for #name #ty_generics
        #where_clause
        {
            const STATE_IDS: &'static [(::crab_protocol::protocol_state::ProtocolState, i32)] = &[(#state, #packet_id)];
        }
    })
}
