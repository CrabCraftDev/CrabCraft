use crate::config::utils::collect_docs;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::Variant;
use syn::punctuated::Punctuated;
use syn::token::Comma;

pub fn generate_enum_code(name_of_config: Ident, variants: Punctuated<Variant, Comma>) -> TokenStream {
    let mut documentation_modifiers = Vec::new();
    for variant in variants.into_iter() {
        let collected = collect_docs(&variant);
        let collected = collected.trim_end_matches('\n');
        let ident = &variant.ident;

        // Note: We use to_string method, to convert enum variant to string in the correct case
        documentation_modifiers.push(quote! {
            lines.push({
                let base = crab_config::to_string(&Self::#ident).unwrap_or_default().trim().to_string();
                if #collected.is_empty() {
                    format!("  - {}", base)
                } else {
                    format!("  - {} -{}", base, #collected)
                }
            });
        });
    }

    quote! {
        impl crab_config::types::config::Config for #name_of_config {
            fn from_configuration_node(node: &crab_config::node::ConfigurationNode) -> Result<Self, ::crab_config::error::ConfigError> {
                let value = node.get_value().ok_or(::crab_config::error::ConfigError::FoundValueExpectedMapping)?;
                let maybe_enum: #name_of_config = crab_config::from_value(value.clone()).map_err(|error| crab_config::error::ConfigError::InvalidFieldValue { serde_error: error })?;
                return Ok(maybe_enum);
            }

            fn as_configuration_node(&self) -> crab_config::node::ConfigurationNode {
                let serde_value = crab_config::to_value(&self).unwrap();

                let mut lines = vec![" Possible values:".to_string()];
                #(#documentation_modifiers)*

                crab_config::node::ConfigurationNode::with_value(serde_value).set_doc(Some(lines.join("\n"))).clone()
            }
        }
    }
}
