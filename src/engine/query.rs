use std::ops::{Deref, DerefMut};

use crate::{
    engine::{
        SceneRegistry,
        component::{Component, ComponentCollection},
    },
    lockable::{LockableReadGuard, LockableWriteGuard},
};

pub struct Read<T: Component> {
    guard: LockableReadGuard<ComponentCollection<T>>,
}

impl<T: Component> Deref for Read<T> {
    type Target = ComponentCollection<T>;

    fn deref(&self) -> &ComponentCollection<T> {
        self.guard.as_ref()
    }
}
pub struct Write<T: Component> {
    guard: LockableWriteGuard<ComponentCollection<T>>,
}

impl<T: Component> Deref for Write<T> {
    type Target = ComponentCollection<T>;

    fn deref(&self) -> &Self::Target {
        self.guard.as_ref()
    }
}

impl<T: Component> DerefMut for Write<T> {
    fn deref_mut(&mut self) -> &mut ComponentCollection<T> {
        self.guard.as_mut()
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
        let guard = reg
            .components
            .read_components()
            .map_err(|_| QueryError::NotAvailable)?;
        Ok(Self { guard })
    }

    fn consume(self, reg: &mut Self::Registry) -> Result<(), QueryError> {
        reg.components
            .read_components_consume(self.guard)
            .map_err(|_| QueryError::CantConsume)
    }
}

impl<T: Component> Query for Write<T> {
    type Registry = SceneRegistry;

    fn prepare(reg: &mut Self::Registry) -> Result<Self, QueryError> {
        Ok(Self {
            guard: reg
                .components
                .write_components()
                .map_err(|_| QueryError::NotAvailable)?,
        })
    }

    fn consume(self, reg: &mut Self::Registry) -> Result<(), QueryError> {
        reg.components
            .write_components_consume(self.guard)
            .map_err(|_| QueryError::CantConsume)
    }
}
