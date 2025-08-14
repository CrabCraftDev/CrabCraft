use crate::config::utils::collect_docs;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::Variant;
use syn::punctuated::Punctuated;
use syn::token::Comma;

pub fn generate_enum_code(name_of_config: Ident, variants: Punctuated<Variant, Comma>) -> TokenStream {
    let mut doc_parts: Vec<String> = vec![" Possible values:".to_string()];

    for variant in variants.into_iter() {
        doc_parts.push(format!("  - {} -{}", variant.ident, collect_docs(&variant)));
    }

    let docs_option = doc_parts.join("\n");

    quote! {
        impl crab_config::types::config::Config for #name_of_config {
            fn from_configuration_node(node: &crab_config::node::ConfigurationNode) -> Result<Self, ::crab_config::error::ConfigError> {
                let value = node.get_value().ok_or(::crab_config::error::ConfigError::FoundValueExpectedMapping)?;
                let maybe_enum: #name_of_config = serde_yml::from_value(value.clone()).map_err(|error| crab_config::error::ConfigError::InvalidFieldValue { serde_error: error })?;
                return Ok(maybe_enum);
            }

            fn as_configuration_node(&self) -> crab_config::node::ConfigurationNode {
                let serde_value = serde_yml::to_value(&self).unwrap();
                crab_config::node::ConfigurationNode::with_value(serde_value).set_doc(Some(#docs_option.to_string()))
            }
        }
    }
}
