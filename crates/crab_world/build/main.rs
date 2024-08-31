use std::path::Path;
use std::process::Command;
use std::{env, fs};

use heck::ToUpperCamelCase;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use serde::Deserialize;

const BLOCKS_OUTPUT_FILE: &str = "blocks.rs";
const BLOCKS: &str = include_str!("../../../assets/blocks.json");

#[derive(Deserialize, Clone, Debug)]
struct Block {
    name: String,
    default_state_offset: u16,
    properties: Vec<Property>,
}

#[derive(Deserialize, Clone, Debug)]
struct Property {
    name: String,
    values: Vec<String>,
}

pub fn generate() -> TokenStream {
    println!("cargo::rerun-if-changed=../../assets/blocks.json");

    let content: Vec<Block> = serde_json::from_str(BLOCKS).unwrap();

    let mut blocks = Vec::new();
    let mut blocks_initialization: Vec<TokenStream> = Vec::new();

    for block in content.iter() {
        let name = format_ident!("{}", &block.name.to_upper_camel_case());
        let default_state_offset = block.default_state_offset;
        let (mut property_names, mut states_combinations_amount, mut match_string_to_number) = process_states(block);

        let states_amount = states_combinations_amount.iter().product::<u16>();

        let block_name = &block.name;

        blocks_initialization.push(quote! {
            blocks.add_block(#states_amount, &#name::namespaced_key(), ::crab_world::block_registry::BlockInformation {
                from_properties: #name::from_properties,
                from_string_properties: #name::from_string_properties,
                properties: #name::properties,
                starting_state_id: 0,
                default_state_id: #default_state_offset
            }).unwrap();
        });

        states_combinations_amount.reverse();
        property_names.reverse();
        match_string_to_number.reverse();

        blocks.push(quote! {
            #[derive(Clone, PartialEq, Hash)]
            pub struct #name;

            impl ::crab_world::BlockType for #name {
                fn namespaced_key() -> ::crab_protocol::types::namespaced_key::NamespacedKey {
                    ::crab_protocol::types::namespaced_key::NamespacedKey::minecraft(#block_name).unwrap()
                }

                #[allow(unused_mut, unused)]
                fn properties(local_state: u16) -> Result<::std::collections::HashMap<&'static str, u16>, ::crab_world::error::WorldError> {
                    if local_state > #states_amount {
                        return Err(::crab_world::error::WorldError::InvalidLocalState(local_state, #states_amount));
                    }

                    let mut hashmap: ::std::collections::HashMap<&'static str, u16> = ::std::collections::HashMap::new();
                    let mut current_state = local_state;

                    #(
                        hashmap.insert(stringify!(#property_names), current_state % #states_combinations_amount);
                        current_state /= #states_combinations_amount;
                    )*

                    Ok(hashmap)
                }

                #[allow(unused_mut, unused)]
                fn from_string_properties(hashmap: &::std::collections::HashMap<&'static str, String>) -> Result<::crab_world::Block, ::crab_world::error::WorldError> {
                    let mut state_id_offset: u16 = Self::starting_state_id()?.0;
                    let mut multiplier: u16 = 1;
                    #(
                        let property_value = #match_string_to_number?;
                        state_id_offset += property_value as u16 * multiplier;
                        multiplier *= #states_combinations_amount;
                    )*
                    Ok(::crab_world::Block(state_id_offset))
                }

                #[allow(unused_mut, unused)]
                fn from_properties(hashmap: &::std::collections::HashMap<&'static str, u16>) -> Result<::crab_world::Block, ::crab_world::error::WorldError> {
                    let mut state_id_offset: u16 = Self::starting_state_id()?.0;
                    let mut multiplier: u16 = 1;
                    #(
                        let value = hashmap.get(stringify!(#property_names))
                            .ok_or_else(|| ::crab_world::error::WorldError::InvalidPropertyName)?;
                        state_id_offset += *value * multiplier;
                        multiplier *= #states_combinations_amount;
                    )*
                    Ok(::crab_world::Block(state_id_offset))
                }
            }
        });
    }

    quote! {
        use ::crab_world::BlockType;

        pub fn init_blocks() -> ::crab_world::block_registry::BlockRegistry {
            let mut blocks = ::crab_world::block_registry::BlockRegistry::default();
            #(#blocks_initialization)*
            blocks
        }

        #(#blocks)*
    }
}

fn process_states(block: &Block) -> (Vec<Ident>, Vec<u16>, Vec<TokenStream>) {
    let mut property_names = vec![];
    let mut match_string_to_number = vec![];

    let mut state_combinations_count = vec![];
    for property in &block.properties {
        state_combinations_count.push(property.values.len() as u16);

        let property_name = property.name.as_str();
        property_names.push(format_ident!("{}", property_name));

        let mut property_value_matches = vec![];
        for (i, value) in property.values.iter().enumerate() {
            property_value_matches.push(quote! {#value => Ok::<usize, crate::error::WorldError>(#i)});
        }
        match_string_to_number.push(quote! {
            match hashmap.get(#property_name) {
                Some(value) => match value.as_str() {
                    #(#property_value_matches,)*
                    _ => return Err(crate::error::WorldError::InvalidPropertyValue),
                },
                None => return Err(crate::error::WorldError::InvalidPropertyName),
            }
        });
    }

    (property_names, state_combinations_count, match_string_to_number)
}

fn main() {
    let output_dir = env::var_os("OUT_DIR").unwrap();
    let blocks_file = Path::new(&output_dir).join(BLOCKS_OUTPUT_FILE);
    fs::write(blocks_file.clone(), generate().to_string()).unwrap();

    Command::new("rustfmt")
        .arg(blocks_file)
        .output()
        .expect("Failed to run rustfmt!");
}
