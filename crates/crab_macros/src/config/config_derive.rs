extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Ident;
use std::iter::Map;

use quote::quote;
use syn::punctuated::{Iter, Punctuated};
use syn::token::Comma;
use syn::{parse_macro_input, Data, DeriveInput, Expr, ExprLit, Field, Fields, Lit, LitStr, Meta, Token, Type};

pub fn config_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name_of_config = input.ident;

    let generated_code = match input.data {
        Data::Struct(data_struct) => {
            let named_fields: Punctuated<Field, Comma> = match data_struct.fields {
                Fields::Named(fields_named) => fields_named.named,
                _ => unimplemented!("Only named fields are supported"),
            };

            let configuration_entry_to_fields_mappings = named_fields.iter().map(|field| {
                let field_name = &field.ident;
                let field_type = &field.ty;
                quote! {
                        #field_name: {
                            let entry = children_map.remove(stringify!(#field_name)).expect(concat!("Missing entry for field: ", stringify!(#field_name)));
                            <#field_type as crab_config::types::config::Config>::map(entry)
                        }
                    }
            });

            let fields_to_configuration_entry_mapping = fields_to_configuration_entry(&named_fields);

            let fields_to_defaults_mapping = fields_to_default(&named_fields);

            let default_creator = create_default(&named_fields, &name_of_config);

            quote! {
                impl crab_config::types::config::Config for #name_of_config {
                    fn map(config_entry: crab_config::configuration_entry::ConfigurationEntry) -> Self {
                        let mut children_map = config_entry
                            .get_next()
                            .cloned()
                            .unwrap_or_default()
                            .into_iter()
                            .map(|entry| (entry.key.clone(), entry))
                            .collect::<std::collections::HashMap<_, _>>();

                        #name_of_config {
                            #(#configuration_entry_to_fields_mappings),*
                        }
                    }

                    fn map_to_configuration_entry(&self) -> Vec<crab_config::configuration_entry::ConfigurationEntry> {
                        let mut vector: Vec<crab_config::configuration_entry::ConfigurationEntry> = Vec::new();
                        #(#fields_to_configuration_entry_mapping)*
                        vector
                    }

                    fn get_default_fields(&self) -> std::collections::HashMap<String, String>  {
                        let mut map: std::collections::HashMap<String,String> = std::collections::HashMap::new();
                        #(#fields_to_defaults_mapping)*
                        return map;
                    }
                }

                impl Default for #name_of_config {
                    fn default() -> Self {
                        #default_creator
                    }
                }
            }
        }
        _ => unimplemented!("Config can only be derived for structs"),
    };

    TokenStream::from(generated_code)
}

fn fields_to_configuration_entry(
    named_fields: &Punctuated<Field, Comma>,
) -> Map<Iter<Field>, fn(&Field) -> proc_macro2::TokenStream> {
    named_fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_ty = &field.ty;

        let is_primitive = is_primitive(field_ty);

        let variables_declaration_expression = quote! {
                    let mut value: core::option::Option<String> = core::option::Option::None;
                    let mut children: core::option::Option<Vec<crab_config::configuration_entry::ConfigurationEntry>> = core::option::Option::None;
                    let mut maybe_some_children: Vec<crab_config::configuration_entry::ConfigurationEntry> = Vec::new();
        };

        let field_value_expression = if is_primitive {
            quote! {
                        value =
                            core::option::Option::Some(String::from(format!("{}", &self.#field_name)));
                    }
        } else {
            quote! {
                        value = core::option::Option::None;
                    }
        };

        let docs = collect_docs(field);
        let docs_option = if docs.is_empty() {
            quote! { core::option::Option::None }
        } else {
            quote! { core::option::Option::Some(#docs.to_owned()) }
        };

        let children_value_expression = if is_primitive {
            quote! {
                        children = core::option::Option::None;
                    }
        } else {
            quote! {
                        maybe_some_children = self.#field_name.map_to_configuration_entry();
                        children =
                        if (!&maybe_some_children.is_empty()) {
                                core::option::Option::Some((&maybe_some_children).clone())
                            }
                            else {
                                core::option::Option::None
                            };
                    }
        };

        quote! {
                    #variables_declaration_expression
                    #children_value_expression
                    #field_value_expression
                    vector.push(crab_config::configuration_entry::ConfigurationEntry::new_with_doc(
                        stringify!(#field_name).to_owned(),
                        children,
                        value,
                        #docs_option,
                        false
                    ).unwrap());
                }
    })
}

fn collect_docs(field: &Field) -> String {
    field
        .attrs
        .iter()
        .filter_map(|attr| {
            if attr.path().is_ident("doc") {
                if let Meta::NameValue(meta) = &attr.meta {
                    if let Expr::Lit(ExprLit {
                        lit: Lit::Str(lit_str), ..
                    }) = &meta.value
                    {
                        return Some(lit_str.value());
                    }
                }
            }
            None
        })
        .collect::<Vec<_>>()
        .join("\n")
}
fn fields_to_default(
    named_fields: &Punctuated<Field, Comma>,
) -> Map<Iter<Field>, fn(&Field) -> proc_macro2::TokenStream> {
    named_fields.iter().map(|field| {
        let default = collect_default_value(field);
        if default.is_empty() {
            return quote! {};
        }
        let field_name = &field.ident;
        quote! {
            map.insert(stringify!(#field_name).to_string(), #default.to_string());
        }
    })
}

fn create_default(named_fields: &Punctuated<Field, Comma>, name_of_config: &Ident) -> proc_macro2::TokenStream {
    let configuration_entries_to_add = named_fields.iter().map(|field| {
        let field_type = &field.ty;
        let field_name = &field.ident;
        let docs = collect_docs(field);
        if !is_primitive(field_type) {
            return quote! {
                let field_type_default: #field_type = <#field_type as Default>::default();
                field_configuration_entries.push(
                crab_config::configuration_entry::ConfigurationEntry::new_with_doc(
                        stringify!(#field_name).to_string(),
                        core::option::Option::Some(<#field_type as crab_config::types::config::Config>::map_to_configuration_entry(&field_type_default)),
                        core::option::Option::None,
                        core::option::Option::Some(#docs.to_string()),
                        false
                    ).unwrap()
                );
            }
        }
        let mut default = collect_default_value(field);
        if default.is_empty() {
            default = "0".to_string();
        }
        quote! {
            field_configuration_entries.push(
                crab_config::configuration_entry::ConfigurationEntry::new_with_doc(
                        stringify!(#field_name).to_string(),
                        core::option::Option::None,
                        core::option::Option::Some(#default.to_string()),
                        core::option::Option::Some(#docs.to_string()),
                        false
                    ).unwrap()
            );
        }
    });

    quote! {
        let mut field_configuration_entries: Vec<crab_config::configuration_entry::ConfigurationEntry> = Vec::new();
        #(#configuration_entries_to_add)*

        <#name_of_config as crab_config::types::config::Config>::map(crab_config::configuration_entry::ConfigurationEntry::new_with_doc(
            "root".to_string(),
            core::option::Option::Some(field_configuration_entries),
            core::option::Option::None,
            core::option::Option::None,
            true
        ).unwrap())
    }
}

fn collect_default_value(field: &Field) -> String {
    field
        .attrs
        .iter()
        .filter_map(|attr| {
            if attr.path().is_ident("default_value") {
                if let Meta::NameValue(meta) = &attr.meta {
                    if let Expr::Lit(ExprLit {
                        lit: Lit::Str(lit_str), ..
                    }) = &meta.value
                    {
                        return Some(lit_str.value());
                    }
                } else if let Meta::List(list) = &attr.meta {
                    if let Ok(parsed_args) = list.parse_args_with(Punctuated::<LitStr, Token![,]>::parse_terminated) {
                        return Some(
                            parsed_args
                                .iter()
                                .map(|it| it.value())
                                .collect::<Vec<String>>()
                                .join(","),
                        );
                    }
                }
            }
            None
        })
        .collect::<String>()
}

fn is_primitive(field_type: &Type) -> bool {
    match field_type {
        Type::Reference(type_reference) => is_primitive(&type_reference.elem),
        Type::Path(type_path) => {
            if let Some(ident) = type_path.path.get_ident() {
                matches!(
                    ident.to_string().as_str(),
                    "i32" | "i64" | "u32" | "u64" | "u8" | "i8" | "f32" | "f64" | "bool" | "char" | "String" | "&str"
                )
            } else {
                false
            }
        }
        _ => false,
    }
}
