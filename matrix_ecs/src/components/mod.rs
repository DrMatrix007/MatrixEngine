use crate::storage::StorageType;
use std::any::TypeId;

pub trait Component: Sized + 'static {
    const STORAGE_TYPE: StorageType = StorageType::Table;
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
pub struct ComponentType(TypeId);

impl ComponentType {
    pub fn from<T: Component>() -> Self {
        ComponentType(TypeId::of::<T>())
    }
}
