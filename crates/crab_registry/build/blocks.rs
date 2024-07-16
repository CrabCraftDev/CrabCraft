use heck::ToUpperCamelCase;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
struct Block {
    name: String,
    default_state: u16,
    properties: Vec<Property>,
}

#[derive(Deserialize, Clone, Debug)]
struct Property {
    name: String,
    values: Vec<String>,
}

const BLOCKS: &str = include_str!("../../../assets/blocks.json");

pub fn generate() -> TokenStream {
    println!("cargo::rerun-if-changed=../../assets/blocks.json");

    let content: Vec<Block> = serde_json::from_str(BLOCKS).unwrap();

    let mut blocks = Vec::new();
    let mut blocks_properties = Vec::new();
    let mut get_block_by_id_arms = Vec::new();

    let mut state_id: u16 = 0;
    for block in content.iter() {
        let name = format_ident!("{}", &block.name.to_upper_camel_case());
        let default_state = block.default_state;
        let (mut properties_names, mut used_property_enums, mut states_combinations_amount) =
            process_states(block, name.clone(), &mut blocks_properties);

        let states_amount = states_combinations_amount.iter().product::<u16>();
        let to_state_id = state_id + states_amount - 1;
        get_block_by_id_arms.push(quote! {
            #state_id..=#to_state_id => {
                Some(Box::new(#name::from_local_state(state_id - #default_state)?))
            }
        });

        let struct_definition = if !properties_names.is_empty() {
            quote! {
                pub struct #name {
                    #(pub #properties_names: #used_property_enums,)*
                }
            }
        } else {
            quote! {
                pub struct #name;
            }
        };

        states_combinations_amount.reverse();
        properties_names.reverse();
        used_property_enums.reverse();

        blocks.push(quote! {
            #[derive(Debug, Clone, PartialEq, Hash)]
            #struct_definition

            impl Block for #name {
                #[allow(unused_mut, unused)]
                fn get_state(&self) -> u16 {
                    let mut state = #state_id;
                    let mut multiplier = 1;
                    #(
                        state += self.#properties_names as u16 * multiplier;
                        multiplier *= #states_combinations_amount;
                    )*
                    state
                }

                #[allow(unused_mut, unused)]
                fn from_local_state(mut state: u16) -> Option<Self> {
                    if state >= #states_amount {
                        return None;
                    }
                    #(
                        let #properties_names = state % #states_combinations_amount;
                        state /= #states_combinations_amount;
                    )*
                    Some(#name {
                        #(#properties_names: #used_property_enums::from_repr(#properties_names as usize)?,)*
                    })
                }
            }

            impl Default for #name {
                fn default() -> Self {
                    #[allow(clippy::identity_op, clippy::eq_op)]
                    Self::from_local_state(#default_state - #state_id).unwrap()
                }
            }
        });

        state_id += states_amount;
    }

    quote! {
        /// Returns a block with correct properties by state id.
        pub fn get_block(state_id: u16) -> Option<Box<dyn Block>> {
            #[allow(clippy::identity_op)]
            match state_id {
                #(#get_block_by_id_arms)*
                _ => None,
            }
        }

        #(#blocks)*

        #(#blocks_properties)*
    }
}

fn get_property_name(property: &Property) -> &str {
    match property.name.as_str() {
        // "type" is reserved keyword in Rust
        "type" => "kind",
        _ => &property.name,
    }
}

fn generate_property_enum(property: &Property, full_name: &Ident) -> TokenStream {
    let property_values = property.values.iter().map(|property| {
        let property = property.to_upper_camel_case();
        if property.parse::<u8>().is_ok() {
            format_ident!("_{}", property)
        } else {
            format_ident!("{}", property)
        }
    });

    quote! {
        #[derive(Debug, Copy, Clone, PartialEq, Hash, strum::FromRepr)]
        pub enum #full_name {
            #(#property_values,)*
        }
    }
}

fn process_states(
    block: &Block,
    name: Ident,
    blocks_properties: &mut Vec<TokenStream>,
) -> (Vec<Ident>, Vec<TokenStream>, Vec<u16>) {
    let mut properties_names = vec![];
    let mut used_property_enums = vec![];

    let mut states_amount = vec![];
    for property in &block.properties {
        states_amount.push(property.values.len() as u16);

        let property_name = get_property_name(property);
        properties_names.push(format_ident!("{}", property_name));

        if property.values == ["true", "false"] {
            used_property_enums.push(quote!(crate::blocks::BooleanProperty));
            continue;
        }

        let block_property_name = format_ident!("{}{}", name, property_name.to_upper_camel_case());
        blocks_properties.push(generate_property_enum(property, &block_property_name));
        used_property_enums.push(block_property_name.to_token_stream());
    }

    (properties_names, used_property_enums, states_amount)
}
