use std::{
    any::{Any, TypeId},
    collections::BTreeMap,
    sync::Arc,
};

use tokio::sync::{OwnedRwLockReadGuard, OwnedRwLockWriteGuard, RwLock};

use super::Resource;

#[derive(Default, Debug)]
pub struct ResourceRegistry {
    data: BTreeMap<TypeId, Box<dyn Any>>,
}

impl ResourceRegistry {
    pub fn try_read<R: Resource + 'static>(
        &mut self,
    ) -> Result<OwnedRwLockReadGuard<ResourceHolder<R>>, tokio::sync::TryLockError> {
        self.data
            .entry(TypeId::of::<R>())
            .or_insert_with(|| Box::new(Arc::new(RwLock::new(ResourceHolder::<R>::default()))))
            .downcast_ref::<Arc<RwLock<ResourceHolder<R>>>>()
            .unwrap()
            .clone()
            .try_read_owned()
    }
    pub fn try_write<R: Resource + 'static>(
        &mut self,
    ) -> Result<OwnedRwLockWriteGuard<ResourceHolder<R>>, tokio::sync::TryLockError> {
        self.data
            .entry(TypeId::of::<R>())
            .or_insert_with(|| Box::new(Arc::new(RwLock::new(ResourceHolder::<R>::default()))))
            .downcast_ref::<Arc<RwLock<ResourceHolder<R>>>>()
            .unwrap()
            .clone()
            .try_write_owned()
    }
}

#[derive(Debug)]
pub struct ResourceHolder<R: Resource> {
    data: Option<R>,
}

impl<R: Resource> AsMut<Option<R>> for ResourceHolder<R> {
    fn as_mut(&mut self) -> &mut Option<R> {
        &mut self.data
    }
}

impl<R: Resource> AsRef<Option<R>> for ResourceHolder<R> {
    fn as_ref(&self) -> &Option<R> {
        &self.data
    }
}

impl<R: Resource> Default for ResourceHolder<R> {
    fn default() -> Self {
        Self {
            data: Default::default(),
        }
    }
}
