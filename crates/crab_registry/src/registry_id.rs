use std::sync::Arc;

use crab_text::utils::namespaced_key::NamespacedKey;

use crate::{REGISTRY, Registrable, registry::Registry};

/// Convenience trait that allows pulling data from global registry
pub trait RegistryAccessExt {
    fn registry_entry<T: Registrable>(&self) -> <Self as RegistryId<T>>::AsValue
    where
        Self: RegistryId<T>,
    {
        self.entry_from_registry(REGISTRY.load().registry_by_type::<T>())
    }

    fn namespaced_key<T: Registrable>(&self) -> <Self as RegistryId<T>>::AsNamespacedKey
    where
        Self: RegistryId<T>,
    {
        self.namespaced_key_from_registry(REGISTRY.load().registry_by_type::<T>())
    }

    fn numeric_id<T: Registrable>(&self) -> <Self as RegistryId<T>>::AsNumericId
    where
        Self: RegistryId<T>,
    {
        self.numeric_id_from_registry(REGISTRY.load().registry_by_type::<T>())
    }
}

impl<T> RegistryAccessExt for T {}

pub trait RegistryId<T: Registrable> {
    type AsValue;
    type AsNumericId;
    type AsNamespacedKey;

    fn entry_from_registry(&self, registry: &Registry<T>) -> Self::AsValue;

    fn numeric_id_from_registry(&self, registry: &Registry<T>) -> Self::AsNumericId;

    fn namespaced_key_from_registry<'ns>(&self, registry: &Registry<T>) -> Self::AsNamespacedKey;
}

impl<T: Registrable> RegistryId<T> for NamespacedKey {
    type AsValue = Option<Arc<T>>;
    type AsNumericId = Option<usize>;
    type AsNamespacedKey = NamespacedKey;

    fn entry_from_registry(&self, registry: &Registry<T>) -> Self::AsValue {
        registry.get_by_key(self)
    }

    fn numeric_id_from_registry(&self, registry: &Registry<T>) -> Self::AsNumericId {
        registry.key_to_id(self)
    }

    fn namespaced_key_from_registry(&self, _registry: &Registry<T>) -> Self::AsNamespacedKey {
        self.clone()
    }
}

impl<T: Registrable> RegistryId<T> for usize {
    type AsValue = Option<Arc<T>>;
    type AsNumericId = usize;
    type AsNamespacedKey = Option<NamespacedKey>;

    fn numeric_id_from_registry(&self, _registry: &Registry<T>) -> Self::AsNumericId {
        *self
    }

    fn entry_from_registry(&self, registry: &Registry<T>) -> Self::AsValue {
        registry.get_by_id(*self)
    }

    fn namespaced_key_from_registry(&self, registry: &Registry<T>) -> Self::AsNamespacedKey {
        registry.id_to_key(*self).cloned()
    }
}

#[cfg(test)]
mod test {
    use std::{str::FromStr, sync::Arc};

    use crab_text::utils::namespaced_key::NamespacedKey;

    use crate::{registry_id::RegistryAccessExt, vanilla::Biome};

    #[test]
    fn api_test() {
        let plains_registry_key = Biome::PLAINS;
        let plains_ns_key = NamespacedKey::from_str(Biome::PLAINS.id).unwrap();
        let plains_numeric_id = 40;

        let nid_to_ns: Option<NamespacedKey> = plains_numeric_id.namespaced_key::<Biome>();
        let rk_to_ns: NamespacedKey = plains_registry_key.namespaced_key();
        let ns_to_ns: NamespacedKey = plains_ns_key.namespaced_key::<Biome>(); // Return self, cloned :|

        let nid_to_nid: usize = plains_numeric_id.numeric_id::<Biome>(); // Return self
        let rk_to_nid: usize = plains_registry_key.numeric_id();
        let ns_to_nid: Option<usize> = plains_ns_key.numeric_id::<Biome>();

        let nid_to_re: Option<Arc<Biome>> = plains_numeric_id.registry_entry::<Biome>();
        let rk_to_re: Arc<Biome> = plains_registry_key.registry_entry();
        let ns_to_re: Option<Arc<Biome>> = plains_ns_key.registry_entry::<Biome>();

        assert_eq!(plains_ns_key, nid_to_ns.unwrap());
        assert_eq!(plains_ns_key, rk_to_ns);
        assert_eq!(plains_ns_key, ns_to_ns);

        assert_eq!(plains_numeric_id, nid_to_nid);
        assert_eq!(plains_numeric_id, rk_to_nid);
        assert_eq!(plains_numeric_id, ns_to_nid.unwrap());

        assert_eq!(nid_to_re.unwrap(), rk_to_re);
        assert_eq!(rk_to_re, ns_to_re.unwrap());
    }
}
