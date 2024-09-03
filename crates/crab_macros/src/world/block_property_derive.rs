use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{parse2, Data, DeriveInput, Error, Fields, Result, Type};

pub(crate) fn derive_block_property_impl(item: TokenStream) -> Result<TokenStream> {
    let input = parse2::<DeriveInput>(item)?;
    let input_name = &input.ident;
    let input_span = input.span();
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => fields.named.iter(),
            _ => return Err(Error::new(input_span, "All fields have to be named!")),
        },
        _ => return Err(Error::new(input_span, "You can use `BlockData` only for structs")),
    };

    let mut field_names = Vec::new();
    let mut property_parsing = Vec::new();
    let mut apply_properties = Vec::new();

    for field in fields {
        let field_name = field.ident.clone().unwrap();
        let field_type = &field.ty;

        field_names.push(field_name.clone());

        let Type::Path(type_path) = field_type else {
            return Err(Error::new(field_type.span(), "Unsupported field type"));
        };

        let type_ident = type_path.path.get_ident().unwrap().to_string();
        let field_conversion = match type_ident.as_str() {
            "u16" => quote! { *value },
            "bool" => quote! {
                match value {
                    0 => true,
                    1 => false,
                    _ => return Err(::crab_world::error::WorldError::InvalidPropertyValue),
                }
            },
            _ => quote! { #field_type::from_repr(*value).unwrap() },
        };

        let apply_property = match type_ident.as_str() {
            "u16" => quote! { self.#field_name },
            "bool" => quote! {
                match self.#field_name {
                    true => 0,
                    false => 1,
                }
            },
            _ => quote! { self.#field_name as u16 },
        };

        property_parsing.push(quote! {
            let value = match properties.get(stringify!(#field_name)) {
                Some(value) => value,
                None => return Err(::crab_world::error::WorldError::InvalidPropertyName),
            };
            let #field_name = #field_conversion;
        });
        apply_properties.push(quote! {
            properties.insert(stringify!(#field_name).to_string(), #apply_property);
        });
    }

    Ok(quote! {
        impl #impl_generics ::crab_world::block_data::BlockData for #input_name #type_generics
        #where_clause
        {
            fn from_properties(properties: ::std::collections::HashMap<String, u16>) -> Result<Self, ::crab_world::error::WorldError> {
                #(#property_parsing)*
                Ok(Self {
                    #(#field_names,)*
                })
            }

            fn apply(&self, properties: &mut ::std::collections::HashMap<String, u16>) -> Result<(), ::crab_world::error::WorldError> {
                #(#apply_properties)*
                Ok(())
            }
        }
    })
}
