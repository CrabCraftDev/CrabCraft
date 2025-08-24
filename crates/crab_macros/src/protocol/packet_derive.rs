use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Result, parse2};

use crate::protocol::args::{RootPacketArgs, parse_packet_args};

pub(crate) fn derive_packet_impl(item: TokenStream) -> Result<TokenStream> {
    let input = parse2::<DeriveInput>(item)?;
    let packet_args = parse_packet_args(&input.attrs)?.expect("Packet attribute missing!");
    let name = input.ident;

    let RootPacketArgs {
        name: packet_name,
        id: packet_id,
        state,
    } = packet_args;

    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics ::crab_protocol::packet::Packet for #name #type_generics
        #where_clause
        {
            const NAME: &'static str = #packet_name;
            const STATE_IDS: &'static [(::crab_protocol::protocol_state::ProtocolState, i32)] = &[(#state, #packet_id)];
        }
    })
}
