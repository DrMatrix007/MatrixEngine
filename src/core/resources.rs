use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::Arc,
};

use tokio::sync::RwLock;

pub trait Resource: 'static {}

pub struct ResourceHolder<R: Resource>(R);

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
            Box::new(Arc::new(RwLock::new(ResourceHolder(r)))),
        );
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
