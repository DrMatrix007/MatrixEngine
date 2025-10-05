use std::ops::{Deref, DerefMut};

use crate::{
    engine::{
        SceneRegistry,
        component::{Component, ComponentCollection},
    },
    lockable::{LockableReadGuard, LockableWriteGuard},
};

pub struct Read<T: Component> {
    data: LockableReadGuard<ComponentCollection<T>>,
}

impl<T: Component> Deref for Read<T> {
    type Target = ComponentCollection<T>;
    
    fn deref(&self) -> &ComponentCollection<T> {
        self.data.as_ref()
    }
}
pub struct Write<T: Component> {
    data: LockableWriteGuard<ComponentCollection<T>>,
}

impl<T: Component> Deref for Write<T> {
    type Target = ComponentCollection<T>;

    fn deref(&self) -> &Self::Target {
        self.data.as_ref()
    }
}

impl<T: Component> DerefMut for Write<T> {
    fn deref_mut(&mut self) -> &mut ComponentCollection<T> {
        self.data.as_mut()
    }
}

#[derive(Debug)]
pub enum QueryError {
    NotAvailable,
    CantConsume,
    ConsumableIsEmpty,
}

pub trait Query: Send + Sized {
    type Registry;

    fn prepare(reg: &mut Self::Registry) -> Result<Self, QueryError>;
    fn consume(self, reg: &mut Self::Registry) -> Result<(), QueryError>;
}

impl<T: Component> Query for Read<T> {
    type Registry = SceneRegistry;

    fn prepare(reg: &mut Self::Registry) -> Result<Self, QueryError> {
        Ok(Self {
            data: reg
                .components
                .read_components()
                .ok_or(QueryError::NotAvailable)?,
        })
    }

    fn consume(self, reg: &mut Self::Registry) -> Result<(), QueryError> {
        reg.components
            .read_components_consume(self.data)
            .map_err(|_| QueryError::CantConsume)
    }
}

impl<'a, T: Component> Query for Write<T> {
    type Registry = SceneRegistry;

    fn prepare(reg: &mut Self::Registry) -> Result<Self, QueryError> {
        Ok(Self {
            data: reg
                .components
                .write_components()
                .ok_or(QueryError::NotAvailable)?,
        })
    }

    fn consume(self, reg: &mut Self::Registry) -> Result<(), QueryError> {
        reg.components
            .write_components_consume(self.data)
            .map_err(|_| QueryError::CantConsume)
    }
}
