use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, spanned::Spanned, Data, DeriveInput, Error, Fields, Result};

pub(crate) fn derive_block_property_impl(item: TokenStream) -> Result<TokenStream> {
    let input = parse2::<DeriveInput>(item)?;
    let input_name = &input.ident;
    let input_span = input.span();
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => return Err(Error::new(input_span, "All fields have to be named!")),
        },
        _ => return Err(Error::new(input_span, "You can use `BlockData` only for structs")),
    };

    let mut field_names = Vec::new();
    let mut property_parsing = Vec::new();
    let mut apply_properties = Vec::new();

    let (value_type_def, get_impl) = if fields.len() == 1 {
        let field_type = &fields[0].ty;
        let field_name = &fields[0].ident;
        (
            quote! {
                type Value = #field_type;
            },
            quote! {
                self.#field_name
            },
        )
    } else {
        (
            quote! { type Value = Self; },
            quote! {
                self
            },
        )
    };

    for field in fields {
        let field_name = field.ident.clone().unwrap();

        field_names.push(field_name.clone());
        let field_get = if fields.len() != 1 {
            quote! { property.#field_name.to_string() }
        } else {
            quote! { property.to_string() }
        };

        property_parsing.push(quote! {
            let value = match properties.get(stringify!(#field_name)) {
                Some(value) => value,
                None => return Err(::crab_world::error::WorldError::InvalidPropertyName),
            };
            let #field_name = value.parse().map_err(|_| ::crab_world::error::WorldError::InvalidPropertyValue)?;
        });

        apply_properties.push(quote! {
            properties.insert(stringify!(#field_name), #field_get.to_lowercase());
        });
    }

    Ok(quote! {
        impl #impl_generics ::crab_world::blocks::block_data::BlockData for #input_name #type_generics
        #where_clause
        {
            #value_type_def

            fn get(self) -> Self::Value {
                #get_impl
            }

            fn from_properties(properties: ::std::collections::HashMap<&str, String>) -> Result<Self, ::crab_world::error::WorldError> {
                #(#property_parsing)*
                Ok(Self {
                    #(#field_names,)*
                })
            }

            fn apply(property: &Self::Value, properties: &mut ::std::collections::HashMap<&str, String>) -> Result<(), ::crab_world::error::WorldError> {
                #(#apply_properties)*
                Ok(())
            }
        }
    })
}
