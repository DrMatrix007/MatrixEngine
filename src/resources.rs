use std::{
    any::{Any, TypeId},
    collections::HashMap,
    default,
};

use crate::registry::ReadError;

#[derive(Default)]
pub struct ResourceManager {
    data: HashMap<TypeId, Box<dyn Any>>,
}

impl ResourceManager {
    pub fn insert_resource<T: 'static>(&mut self, t: T) {
        self.data.insert(TypeId::of::<T>(), Box::new(t));
    }
    pub fn get_resource<T: 'static>(&self) -> Result<&T, ReadError> {
        let Some(a) = self.data.get(&TypeId::of::<T>()) else {
            return Err(ReadError::NotExist);
        };
        match a.downcast_ref::<T>() {
            Some(a) => Ok(a),
            None => Err(ReadError::CantRead),
        }
    }
}
