use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{DeriveInput, Error, Path, Result, Type, parse2, spanned::Spanned};

pub(crate) fn detailed_from_impl(item: TokenStream) -> Result<TokenStream> {
    let input = parse2::<DeriveInput>(item)?;
    let input_name = &input.ident;

    let struct_name = format_ident!("Detailed{input_name}");

    let (fields, detailed_fields) = collect_fields(&input)?;

    let output = quote! {

        #[derive(Debug)]
        pub struct #struct_name {
            pub error: #input_name,
            pub details: ::crab_utils::error::ErrorDetails,
        }

        impl From<#input_name> for #struct_name {
            #[track_caller]
            fn from(error: #input_name) -> Self {
                Self {
                    error,
                    details: ::crab_utils::error::ErrorDetails::new(),
                }
            }
        }

        impl std::fmt::Display for #struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "Error: {}\n{}", self.error, self.details)
            }
        }

        #(
            impl From<#fields> for #struct_name {
                #[track_caller]
                fn from(value: #fields) -> Self {
                    Self {
                        error: #input_name::from(value),
                        details: ::crab_utils::error::ErrorDetails::new(),
                    }
                }
            }
        )*

        #(
            impl From<#detailed_fields> for #struct_name {
                #[track_caller]
                fn from(value: #detailed_fields) -> Self {
                    Self {
                        error: #input_name::from(value.error),
                        details: value.details,
                    }
                }
            }
        )*
    };

    Ok(output)
}

// Extract a list of fields that have a #[from] attribute
fn collect_fields(input: &DeriveInput) -> Result<(Vec<&Type>, Vec<Path>)> {
    let syn::Data::Enum(data_enum) = &input.data else {
        return Err(Error::new(input.span(), "DetailedFrom only works for enums"));
    };
    let mut with_from = vec![];
    let mut with_detailed = vec![];
    let fields = data_enum.variants.iter().flat_map(|variant| &variant.fields);
    for field in fields {
        if field.attrs.iter().any(|attr| attr.path().is_ident("from")) {
            with_from.push(&field.ty);
        }
        if let Some(attr) = field.attrs.iter().find(|attr| attr.path().is_ident("detailed")) {
            let meta = attr.meta.require_list()?;
            let path: Path = meta.parse_args()?;
            with_detailed.push(path);
        }
    }
    Ok((with_from, with_detailed))
}
