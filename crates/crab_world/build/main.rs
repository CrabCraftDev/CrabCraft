use std::path::Path;
use std::process::Command;
use std::{env, fs};

use heck::ToUpperCamelCase;
use proc_macro2::TokenStream;
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
    // Current dir is crates/crab_world/ instead of crates/crab_world/build
    println!("cargo::rerun-if-changed=../../assets/blocks.json");

    let content: Vec<Block> = serde_json::from_str(BLOCKS).unwrap();

    let mut blocks = Vec::new();
    let mut blocks_initialization: Vec<TokenStream> = Vec::new();

    for block in content.iter() {
        let name = format_ident!("{}", &block.name.to_upper_camel_case());
        let default_state_offset = block.default_state_offset;
        let (mut states_combinations_amounts, mut string_to_id, mut id_to_string) = process_states(block);

        let states_amount = states_combinations_amounts.iter().product::<u16>();
        let block_name = &block.name;

        blocks_initialization.push(quote! {
            blocks.add_block(#states_amount, &#name::namespaced_key(), BlockInformation {
                from_properties: #name::from_properties,
                properties: #name::properties,
                starting_state_id: 0,
                default_state_id: #default_state_offset
            }).unwrap();
        });

        states_combinations_amounts.reverse();
        string_to_id.reverse();
        id_to_string.reverse();

        blocks.push(quote! {
            #[derive(Clone, PartialEq, Hash)]
            pub struct #name;

            impl BlockType for #name {
                fn namespaced_key() -> NamespacedKey {
                    NamespacedKey::minecraft(#block_name).unwrap()
                }

                #[allow(unused_mut, unused)]
                fn properties(local_state: u16) -> Result<HashMap<&'static str, String>, WorldError> {
                    if local_state > #states_amount {
                        return Err(WorldError::InvalidLocalState(local_state, #states_amount));
                    }

                    let mut hashmap: HashMap<&'static str, String> = HashMap::new();
                    let mut current_state = local_state;

                    #(
                        let state_id = current_state % #states_combinations_amounts;
                        #id_to_string
                        current_state /= #states_combinations_amounts;
                    )*

                    Ok(hashmap)
                }

                #[allow(unused_mut, unused)]
                fn from_properties(hashmap: &HashMap<&str, String>) -> Result<Block, WorldError> {
                    let mut state_id_offset: u16 = Self::starting_state_id()?.0;
                    let mut multiplier: u16 = 1;
                    #(
                        let property_value = #string_to_id?;
                        state_id_offset += property_value as u16 * multiplier;
                        multiplier *= #states_combinations_amounts;
                    )*
                    Ok(Block(state_id_offset))
                }
            }
        });
    }

    quote! {
        use crab_protocol::types::namespaced_key::NamespacedKey;
        use std::collections::HashMap;
        use crab_world::blocks::block_registry::BlockInformation;

        pub fn init_blocks() -> BlockRegistry {
            let mut blocks = BlockRegistry::default();
            #(#blocks_initialization)*
            blocks
        }

        #(#blocks)*
    }
}

fn process_states(block: &Block) -> (Vec<u16>, Vec<TokenStream>, Vec<TokenStream>) {
    let mut states_combinations_amounts = vec![];
    let mut string_to_id = vec![];
    let mut id_to_string = vec![];

    for property in &block.properties {
        states_combinations_amounts.push(property.values.len() as u16);

        let property_name = property.name.as_str();

        let mut string_to_id_matches = vec![];
        for (i, value) in property.values.iter().enumerate() {
            string_to_id_matches.push(quote! {#value => Ok::<usize, crate::error::WorldError>(#i)});
        }
        string_to_id.push(quote! {
            match hashmap.get(#property_name) {
                Some(value) => match value.as_str() {
                    #(#string_to_id_matches,)*
                    _ => return Err(crate::error::WorldError::InvalidPropertyValue),
                },
                None => return Err(crate::error::WorldError::InvalidPropertyName),
            }
        });

        let mut id_to_string_matches = vec![];
        for (i, value) in property.values.iter().enumerate() {
            let i = i as u16;
            id_to_string_matches.push(quote! {#i => #value.to_string()});
        }
        id_to_string.push(quote! {
            hashmap.insert(#property_name, match state_id {
                #(#id_to_string_matches,)*
                _ => unreachable!()
            });
        });
    }

    (states_combinations_amounts, string_to_id, id_to_string)
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
