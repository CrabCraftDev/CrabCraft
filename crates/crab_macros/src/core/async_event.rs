use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, DeriveInput, Result};

pub(crate) fn async_event_impl(item: TokenStream) -> Result<TokenStream> {
    let input = parse2::<DeriveInput>(item)?;
    let input_name = &input.ident;

    let event_enum_name = input_name.to_string().replace("Event", "");
    let event_enum_ident = syn::Ident::new(&event_enum_name, input_name.span());

    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

    let output = quote! {
        impl #impl_generics TryFrom<AsyncEvent #type_generics> for #input_name #type_generics #where_clause {
            type Error = Error;

            fn try_from(value: AsyncEvent #type_generics) -> Result<Self, Self::Error> {
                match value {
                    AsyncEvent::#event_enum_ident(event) => Ok(event),
                    _ => Err(Error::InvalidAsyncEventType),
                }
            }
        }
    };

    Ok(output)
}
