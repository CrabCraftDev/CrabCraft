use crab_registry::REGISTRY;
use crab_registry::minecraft::Biome;
use crab_registry::registry::{RegistryBase, TypedRegistryBase};
use crab_text::utils::namespaced_key::NamespacedKey;
use std::str::FromStr;
use std::sync::{Arc, Barrier};
use std::thread;

#[test]
fn test_registry_stress() {
    let num_threads = 10;
    let iterations = 200;
    let barrier = Arc::new(Barrier::new(num_threads));

    let handles: Vec<_> = (0..num_threads)
        .map(|thread_id| {
            let barrier = Arc::clone(&barrier);
            thread::spawn(move || {
                barrier.wait();

                for i in 0..iterations {
                    let root = REGISTRY.load();

                    // Vary operations based on thread and iteration
                    match (thread_id + i) % 5 {
                        0 => {
                            // Key lookup
                            let biome_registry = root.registry_by_type::<Biome>();
                            let plains_key = NamespacedKey::from_str("minecraft:plains").unwrap();
                            let _ = biome_registry.value_by_key(&plains_key);
                        }
                        1 => {
                            // ID conversion
                            let biome_registry = root.registry_by_type::<Biome>();
                            let plains_key = NamespacedKey::from_str("minecraft:plains").unwrap();
                            if let Some(id) = biome_registry.key_to_id(&plains_key) {
                                let _ = biome_registry.id_to_key(id);
                            }
                        }
                        2 => {
                            // Registry key conversion
                            let _ = Biome::PLAINS.to_numeric_id();
                            let _ = Biome::THE_VOID.to_numeric_id();
                        }
                        3 => {
                            // Multiple registry access
                            let _ = root.registry_by_type::<Biome>();
                            let _ = root.registry_by_type::<crab_registry::minecraft::DamageType>();
                        }
                        _ => {
                            // Enumerate registries
                            let all = root.all_registries();
                            assert!(all.len() > 0);
                        }
                    }
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().expect("Thread panicked");
    }
}
