use std::marker::PhantomData;

#[derive(Clone)]
pub struct RegistryKey<T> {
    // FIXME: Use NamespacedKey, but we cannot do it for now, because it is not const
    pub id: &'static str,
    _marker: PhantomData<T>,
}

impl<T> RegistryKey<T> {
    pub const fn new(id: &'static str) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }
}
