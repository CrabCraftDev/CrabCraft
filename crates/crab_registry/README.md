# Crab Registry

A central, type-safe, data-driven registry architecture for CrabCraft inspired by [Minecraft's registry](https://minecraft.wiki/w/Java_Edition_protocol/Registry_data) model. 

### Key features

- [X] Each registry is bound to a concrete Rust type at compile time.
- [X] Stable, deterministic insertion order via `ordermap::OrderMap`.
- [X] Fast lookup by namespaced key or numeric ID.
- [X] Distributed population using `linkme`, allowing any crate to contribute registry data.
- [X] Centralized runtime access through a global `RootRegistry` backed by `ArcSwap`.
- [X] Support for multiple registry kinds like `SimpleRegistry` or `MultiIdRegistry`
- [ ] Querying, parsing and grouping registry entries by [tags](https://minecraft.wiki/w/Tag_(Java_Edition)) ([#163](https://github.com/CrabCraftDev/CrabCraft/issues/163))

## Vanilla population and datapack layout

Official data from vanilla Minecraft server is stored inside `assets/datapack/` directory.
The [`vanilla_populator`](src/minecraft.rs) module loads these files and populates the corresponding registries.

Inside the [`vanilla/`](src/minecraft/) directory you can find all the structs corresponding to the vanilla datapack.

> [!NOTE]  
> In the future we want to improve our vanilla datapack loader, to support loading datapacks during the runtime of the game.

## Code Examples

### Adding your own types to a registry
You can add new registry-backed types from any crate in the workspace.

1) Define the value type and bind it to a registry
```rust,ignore
use crab_registry::{Registrable};
use crab_registry::registry_key::RegistryKey;

#[derive(Debug)]
pub struct Biome { pub name: &'static str }

impl Registrable for Biome {
    // Use `SimpleRegistry` for most common cases
    type RegistryType = SimpleRegistry<Self>;

    // This declares which registry holds `Biome`s
    const REGISTRY: RegistryKey<Self> = RegistryKey::new("minecraft:worldgen/biome");
}

// Optional handy keys for common entries
impl Biome {
    pub const PLAINS: RegistryKey<Self> = RegistryKey::new("minecraft:plains");
}
```

2) Add your own populator using the distributed slice:
```rust,ignore
use crab_registry::{RootRegistry, registry::Registry, error::RegistryError REGISTRY_POPULATORS};
use crab_registry::registry_key::RegistryKey;
use crab_registry::Registrable;
use linkme::distributed_slice;
use crab_text::utils::namespaced_key::NamespacedKey;
use std::str::FromStr;

#[distributed_slice(REGISTRY_POPULATORS)]
fn my_populator(root: &mut RootRegistry) -> Result<(), RegistryError> {
    // Create and fill the typed registry
    let mut biomes = SimpleRegistry::<Biome>::default();
    biomes.register(NamespacedKey::from_str(Biome::PLAINS.id).unwrap(), Biome { name: "Plains" });

    // Register it in the root under its registry key
    root.register_registry(Biome::REGISTRY, biomes);
    
    Ok(())
}
```

That’s it: the registry will be available when `RootRegistry::populated()` runs.
