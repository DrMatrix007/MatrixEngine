use std::{
    any::{Any, TypeId},
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use super::data_state::{DataState, DataStateAccessError, ReadDataState, WriteDataState};

pub trait Resource: Send + 'static {}

impl<T: Send + ?Sized + 'static> Resource for T {}

#[derive(Debug)]
pub struct ResourceRegistry {
    data: HashMap<TypeId, Box<dyn Any + Send>>,
}

pub struct ResourceHolder<R: Resource> {
    data: Option<R>,
}

impl<R: Resource> Deref for ResourceHolder<R> {
    type Target = Option<R>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<R: Resource> DerefMut for ResourceHolder<R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<R: Resource> Default for ResourceHolder<R> {
    fn default() -> Self {
        Self { data: Option::None }
    }
}

impl ResourceRegistry {
    pub fn new() -> Self {
        Self {
            data: HashMap::default(),
        }
    }
    fn get_state<C: Resource>(&mut self) -> &mut DataState<ResourceHolder<C>> {
        self.data
            .entry(TypeId::of::<C>())
            .or_insert_with(|| Box::new(DataState::<ResourceHolder<C>>::default()))
            .downcast_mut::<DataState<ResourceHolder<C>>>()
            .expect("Failed to downcast data.")
    }

    pub fn read<C: Resource>(
        &mut self,
    ) -> Result<ReadDataState<ResourceHolder<C>>, DataStateAccessError> {
        self.get_state::<C>().read()
    }

    pub fn write<C: Resource>(
        &mut self,
    ) -> Result<WriteDataState<ResourceHolder<C>>, DataStateAccessError> {
        self.get_state::<C>().write()
    }

    pub fn consume_read<C: Resource>(
        &mut self,
        read: ReadDataState<ResourceHolder<C>>,
    ) -> Result<(), DataStateAccessError> {
        self.get_state::<C>().consume_read(read)
    }

    pub fn consume_write<C: Resource>(
        &mut self,
        write: WriteDataState<ResourceHolder<C>>,
    ) -> Result<(), DataStateAccessError> {
        self.get_state::<C>().consume_write(write)
    }

    pub fn check_read<C: Resource>(&mut self) -> bool {
        self.get_state::<C>().can_read()
    }

    pub fn check_write<C: Resource>(&mut self) -> bool {
        self.get_state::<C>().can_write()
    }

    pub fn try_insert<C: Resource>(&mut self, c: C) -> Result<(), DataStateAccessError> {
        let w = self.get_state::<C>().get_mut()?;
        **w = Some(c);
        Ok(())
    }
}

impl Default for ResourceRegistry {
    fn default() -> Self {
        Self::new()
    }
}
