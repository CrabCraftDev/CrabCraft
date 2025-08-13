extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Ident;
use std::iter::Map;

use quote::{ToTokens, quote};
use syn::punctuated::{Iter, Punctuated};
use syn::token::Comma;
use syn::{Data, DeriveInput, Expr, ExprLit, Field, Fields, Lit, Meta, parse_macro_input};

pub fn config_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name_of_config = input.ident;

    let generated_code = match input.data {
        Data::Struct(data_struct) => {
            let named_fields: Punctuated<Field, Comma> = match data_struct.fields {
                Fields::Named(fields_named) => fields_named.named,
                _ => unimplemented!("Only named fields are supported"),
            };

            let mut elements = Vec::new();

            let configuration_node_to_fields_mappings = named_fields.iter().map(|field| {

                let field_name = &field.ident;
                elements.push(quote! {
                    #field_name: #field_name.unwrap()
                });
                let field_type = &field.ty;
                quote! {
                    let entry = children_map.remove(stringify!(#field_name));
                    let #field_name = match entry {
                        Some(entry) => match <#field_type as crab_config::types::config::Config>::from_configuration_node(&entry) {
                            Ok(value) => Some(value),
                            Err(error) => {
                                errors.push(error);
                                None
                            }
                        },
                        None => {
                            errors.push(crab_config::error::ConfigError::MissingNode { key: stringify!(#field_name).to_string(), parent_key: "root".to_string() });
                            None
                        }
                    };
                }
            });

            let fields_to_configuration_node_mapping = fields_to_configuration_node(&named_fields);

            let default_creator = create_default(&named_fields, &name_of_config);

            quote! {
                impl crab_config::types::config::Config for #name_of_config {
                    fn from_configuration_node(node: &crab_config::node::ConfigurationNode) -> Result<Self, ::crab_config::error::ConfigError> {
                        let mut children_map = node
                            .get_children()
                            .cloned()
                            .unwrap_or_default()
                            .into_iter()
                            .map(|(key, child_node)| (key.clone(), child_node))
                            .collect::<std::collections::HashMap<_, _>>();

                        let mut errors = Vec::new();

                        #(#configuration_node_to_fields_mappings)*

                        if errors.len() > 0 {
                            return Err(::crab_config::error::ConfigError::InvalidConfig { errors });
                        }

                        Ok(#name_of_config {
                            #(#elements),*
                        })
                    }

                    fn as_configuration_node(&self) -> crab_config::node::ConfigurationNode {
                        let mut child = Vec::new();
                        #(#fields_to_configuration_node_mapping)*
                        crab_config::node::ConfigurationNode::with_children(child)
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

fn fields_to_configuration_node(
    named_fields: &Punctuated<Field, Comma>,
) -> Map<Iter<'_, Field>, fn(&Field) -> proc_macro2::TokenStream> {
    named_fields.iter().map(|field| {
        let field_name = &field.ident;

        let mut docs = collect_docs(field);

        let docs_option = if let Some(default) = collect_default_value(field) {
            let default_tokens = default.to_token_stream();
            if !docs.is_empty() {
                docs.push('\n');
            }
            docs.push_str(" Default value: {:?}");

            quote! { Some(format!(#docs, #default_tokens)) }
        } else if docs.is_empty() {
            quote! { None }
        } else {
            quote! { Some(#docs.to_owned()) }
        };

        quote! {
            child.push((stringify!(#field_name).to_owned(), self.#field_name.as_configuration_node().set_doc(#docs_option)));
        }
    })
}

fn collect_docs(field: &Field) -> String {
    field
        .attrs
        .iter()
        .filter_map(|attr| {
            if attr.path().is_ident("doc")
                && let Meta::NameValue(meta) = &attr.meta
                && let Expr::Lit(ExprLit {
                    lit: Lit::Str(lit_str), ..
                }) = &meta.value
            {
                return Some(lit_str.value());
            }
            None
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn create_default(named_fields: &Punctuated<Field, Comma>, name_of_config: &Ident) -> proc_macro2::TokenStream {
    let configuration_entries_to_add = named_fields.iter().map(|field| {
        let field_type = &field.ty;
        let field_name = &field.ident;
        let docs = collect_docs(field);

        let default = if let Some(default_value) = collect_default_value(field) {
            quote! { #default_value }
        } else {
            quote! { #field_type::default() }
        };

        quote! {
            let mut node = <#field_type as crab_config::types::config::Config>::as_configuration_node(&#default);
            node.documentation = Some(#docs.to_string());
            field_configuration_entries.push((stringify!(#field_name).to_string(), node));
        }
    });

    quote! {
        let mut field_configuration_entries = Vec::new();
        #(#configuration_entries_to_add)*

        <#name_of_config as crab_config::types::config::Config>::from_configuration_node(&crab_config::node::ConfigurationNode::with_children(
            field_configuration_entries
        )).unwrap()
    }
}

fn collect_default_value(field: &Field) -> Option<Expr> {
    for attr in &field.attrs {
        if !attr.path().is_ident("default_value") {
            continue;
        }

        if let Meta::List(meta) = &attr.meta
            && let Ok(val) = meta.parse_args::<Expr>()
        {
            return Some(val);
        }
    }

    None
}
