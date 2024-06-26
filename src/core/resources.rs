use std::{
    any::{Any, TypeId}, collections::HashMap, ops::{Deref, DerefMut}, sync::Arc
};

use tokio::sync::RwLock;

pub trait Resource: 'static {}

impl<T:'static> Resource for T{}

pub struct ResourceHolder<R: Resource>(Option<R>);

impl<R:Resource> Deref for ResourceHolder<R> {
    type Target = Option<R>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<R:Resource> DerefMut for ResourceHolder<R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

unsafe impl<R: Resource> Send for ResourceHolder<R> {}

pub struct ResourceRegistry {
    data: HashMap<TypeId, Box<dyn Any>>,
}

impl ResourceRegistry {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
    pub fn add_resource<R: Resource>(&mut self, r: R) {
        self.data.insert(
            TypeId::of::<R>(),
            Box::new(Arc::new(RwLock::new(ResourceHolder(Some(r))))),
        );
    }
    pub fn get_or_insert<R: Resource>(&mut self) -> &Arc<RwLock<ResourceHolder<R>>> {
        unsafe {
            self.data
                .entry(TypeId::of::<R>())
                .or_insert_with(|| Box::new(Arc::new(RwLock::new(ResourceHolder::<R>(None)))))
                .downcast_ref_unchecked()
        }
    }

    pub fn get<R: Resource>(&self) -> Option<&Arc<RwLock<ResourceHolder<R>>>> {
        self.data
            .get(&TypeId::of::<R>())
            .map(|x| unsafe { x.downcast_ref_unchecked() })
    }
}

impl Default for ResourceRegistry {
    fn default() -> Self {
        Self::new()
    }
}
