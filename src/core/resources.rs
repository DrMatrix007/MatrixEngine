use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

pub trait Resource {}

pub struct ResourceHolder<R: Resource>(R);

pub struct ResourceRegistry {
    data: HashMap<TypeId, Box<dyn Any>>,
}

impl ResourceRegistry {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
    
}

impl Default for ResourceRegistry {
    fn default() -> Self {
        Self::new()
    }
}
