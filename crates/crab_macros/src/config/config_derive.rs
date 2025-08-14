extern crate proc_macro;

use proc_macro::TokenStream;

use crate::config::generate_enum::generate_enum_code;
use crate::config::generate_struct::generate_struct_code;
use syn::{Data, DataEnum, DeriveInput, parse_macro_input};

pub fn config_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name_of_config = input.ident;

    let generated_code = match input.data {
        Data::Struct(data_struct) => generate_struct_code(data_struct, name_of_config),
        Data::Enum(DataEnum { variants, .. }) => generate_enum_code(name_of_config, variants),
        _ => unimplemented!("Config can only be derived for structs and unit enums"),
    };

    TokenStream::from(generated_code)
}
