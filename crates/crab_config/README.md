# Crab Config

Advanced type-safe, node-based with versioning and migration system configuration system for CrabCraft.

Based on Java libraries [Configurate](https://github.com/SpongePowered/Configurate) and [BoostedYAML](https://dejvokep.gitbook.io/boostedyaml)

> [!NOTE]  
> At the moment, there is no YAML library that fully supports both comments and Serde. As a result, our current implementation uses a bit cheaty workarounds.

## Documentation

### Setup
First to define a config you need to add a couple of traits
```rust
use crab_macros::Config;
use serde::{Deserialize, Serialize};

#[derive(Config, Serialize, Deserialize)]
struct ExampleConfig {
    _version: u32
}
```
This is basic structure of config it needs to have Config macro and Deserialize, Serialize macro from serde 

Without serialize and deserialize macro the config won't save

**Also needs a field named version for tracking version of config without it config will crash because converter won't have the knowledge to apply patches to the config**  
  
Version field name is defined in: `configurable.rs` under the name of const variable: `VERSION_CONFIG_FIELD_NAME`

### Core concepts
- **Node** - A node is a single value in the config tree. It can be a primitive type, a list, or a map.
- **Migration** - A migration is a function that takes a config and returns a new config. It is used to update the config to a new version.

### Migration Version

> This documentation applies only for the CrabCraft project. For other projects, you can use any versioning scheme you like.

CrabCraft uses a strict versioning scheme for configuration migrations to ensure compatibility and safe upgrades. The format is:

```txt
MAJOR.MINOR.PATCH-migration.<MIGRATION>
```

* **MAJOR.MINOR.PATCH** – Matches the version of the CrabCraft binary.
* **MIGRATION** – A sequential index representing a specific migration step for this binary version. Each migration transforms the config from one version to the next, allowing the system to safely apply incremental updates.

Each migration should be placed in a separate file inside the `src/config/migrations/` directory. File names must follow this pattern:
```txt
m_<MAJOR>_<MINOR>_<PATCH>_<MIGRATION>_<VERY SHORT DESCRIPTION>.rs
```
