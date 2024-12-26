# Crab Config
System for handling configuration files in CrabCraft.

Based on Java libraries [Configurate](https://github.com/SpongePowered/Configurate) and [BoostedYAML](https://dejvokep.gitbook.io/boostedyaml)

## Documentation

### Setup
First to define a config you need to add a couple of traits
```rust
use crab_macros::Config;
use serde::{Deserialize, Serialize};

#[derive(Config, Serialize, Deserialize)]
struct ExampleConfig {
    version: u32
}
```
This is basic structure of config it needs to have Config macro and Deserialize, Serialize macro from serde 

Without serialize and deserialize macro the config won't save

**Also needs a field named version for tracking version of config without it config will crash because converter won't have the knowledge to apply patches to the config**  
  
Version field name is defined in: `configurable.rs` under the name of const variable: `VERSION_CONFIG_FIELD_NAME`

### Usage
If you have some changes you want to apply to specific version for example conversion of some name in config you need to use processor  
Implementing a processor looks the following:
```rust
fn main() {
    let example_processor = Processor::new(
        1, // Config version that we want to alter 
        // For example if our config is from version 0
        // and we want to upgrade to version 1
        // Processor will fire to convert to version 1
        Box::new(|x: &mut ConfigurationEntry| -> &mut ConfigurationEntry {
            x.set_value_by_flat_path("version".to_string(), "1".to_string());
            x.set_value_by_flat_path("test".to_string(), "69420".to_string());
            x
        }), // Convert function that will be used to convert in basic configuration entry
    );
}
```
Why use processors?  
processors will allow to modify current configs on the server to newer format and therefore will allow to load different typed configs into server whilst converting them

Using Configs is fairly simple all you need to do is to use `Configurable` struct like in the following  
Example:
```rust
use crab_macros::Config;
use crate::Configurable;
use serde::{Deserialize, Serialize};

#[derive(Config, Serialize, Deserialize)]
struct MyConfig {
    #[default_value(0)]
    version: u32,
    // Default value used to during saving defaults
    #[default_value(69420)]
    some_thing: u32 
}
fn main() {
    let example_processor = Processor::new(
        1,
        Box::new(|x: &mut ConfigurationEntry| -> &mut ConfigurationEntry {
            x.set_value_by_flat_path("version".to_string(), "1".to_string());
            x
        }),
    );
    
    let configurable = Configurable
        ::<MyConfig> // Our config file type
        ::new("FILE_PATH_TO_READ.yml".to_string(), // Config file path we want to read
              Box::new(YamlHandler {}) // From what handler we want to read in our example we're using .yml so we're using YamlHandler
        );
    // There are several methods to use it with every method is explained below
    let my_config = configurable
        // Read and save default if config doesn't exists
        // Does as said in the function if the config doesn't exists
        // We save the defaults defined in config
        // Otherwise if there's config found function will try to read it using
        // read_config_and_verify function
        .read_and_save_default_if_config_doesnt_exists(vec![
            example_processor
        ]);
    // If there wasn't config found the result should be:
    // 69420
    // because we setup default value of 69420
    // There can also be an error either related to writing or something else
    // as for all listed functions here
    println!("{}", my_config.unwrap().some_thing);
    
    let my_config = configurable
        // Will read config and verify 
        // it's latest version converting it using processors
        .read_config_and_verify(vec![
            example_processor 
        ]);
}
```

Saving also will save any documentation we write or change
```rust
#[derive(Config, Serialize, Deserialize)]
struct MyConfig {
    #[default_value(0)]
    version: u32,
    /// SOME TEST VALUE
    #[default_value(69420)]
    some_thing: u32 
}
fn main() {
    let configurable = Configurable::<MyConfig>::new("FILE_PATH_TO_READ.yml".to_string(),Box::new(YamlHandler {}));
    let my_config = configurable
        .read_and_save_default_if_config_doesnt_exists();
}
```
This will save this into following yaml format:
```yml
version: 0
# SOME TEST VALUE
some_thing: 69420
```

⚠️ **IMPORTANT WARNING**
Using `write_config` has significant limitations:
- Changes are not guaranteed to be persistent
- Modifying keys can corrupt the configuration
- Recommended only for updating values and documentation

For structural changes, use the processor system instead.
```rust
#[derive(Config, Serialize, Deserialize)]
struct MyConfig {
    #[default_value(0)]
    version: u32,
    /// SOME TEST VALUE
    #[default_value(69420)]
    some_thing: u32 
}
fn main() {
    let configurable = Configurable::<MyConfig>::new("FILE_PATH_TO_READ.yml".to_string(),Box::new(YamlHandler {}));
    let my_config = configurable
        .read_and_save_default_if_config_doesnt_exists();
    // This function will map it into a tree more friendly for saving
    let configuration_entries = my_config.as_configuration_entry();
    // Because the function above will give us Vector we need to convert it into root node
    // root node will be the main head of the tree and is constantly used for the saving and reading process
    let root_configuration_entry = ConfigurationEntry::new(
        "root", // key doesn't really matter because it doesn't use at all it with root nodes
        Some(configuration_entries), // We have to set children to this
        None, // Value has to be none otherwise it will panic
        true // is_root has to be true otherwise it will probably malfunction
    );
    configurable.write_config(root_configuration_entry);
}
```