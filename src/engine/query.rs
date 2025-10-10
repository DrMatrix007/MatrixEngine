use std::ops::{Deref, DerefMut};

use crate::{
    engine::{
        EngineState,
        commands::CommandError,
        component::{Component, ComponentCollection},
        resources::{Resource, ResourceHolder},
        system_registries::Stage,
    },
    impl_all,
    utils::lockable::{LockableError, LockableReadGuard, LockableWriteGuard},
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
    LockableError(LockableError),
    CommandError(CommandError),
    TupleError,
}

pub trait Query<Registry>: Send + Sized {
    fn prepare(reg: &mut Registry) -> Result<Self, QueryError>;
    fn consume(self, reg: &mut Registry) -> Result<(), QueryError>;
}

impl<T: Component> Query<EngineState> for Read<T> {
    fn prepare(reg: &mut EngineState) -> Result<Self, QueryError> {
        let guard = reg
            .registry
            .components
            .read()
            .map_err(QueryError::LockableError)?;
        Ok(Self { guard })
    }

    fn consume(self, reg: &mut EngineState) -> Result<(), QueryError> {
        reg.registry
            .components
            .read_consume(self.guard)
            .map_err(QueryError::LockableError)
    }
}

impl<T: Component> Query<EngineState> for Write<T> {
    fn prepare(reg: &mut EngineState) -> Result<Self, QueryError> {
        Ok(Self {
            guard: reg
                .registry
                .components
                .write()
                .map_err(QueryError::LockableError)?,
        })
    }

    fn consume(self, reg: &mut EngineState) -> Result<(), QueryError> {
        reg.registry
            .components
            .write_consume(self.guard)
            .map_err(QueryError::LockableError)
    }
}

pub struct Res<T: Resource> {
    guard: LockableWriteGuard<ResourceHolder<T>>,
}

impl<T: Resource> Res<T> {
    pub fn new(data: LockableWriteGuard<ResourceHolder<T>>) -> Self {
        Self { guard: data }
    }
    pub fn as_ref(&self) -> Option<&T> {
        self.guard.as_ref().as_ref()
    }

    pub fn as_mut(&mut self) -> Option<&mut T> {
        self.guard.as_mut().as_mut()
    }
    pub fn replace(&mut self, data: T) -> Option<T> {
        self.guard.replace(data)
    }
}

impl<T: Resource + 'static> Query<EngineState> for Res<T> {
    fn prepare(reg: &mut EngineState) -> Result<Self, QueryError> {
        Ok(Self::new(
            reg.resources
                .write::<T>()
                .map_err(QueryError::LockableError)?,
        ))
    }

    fn consume(self, reg: &mut EngineState) -> Result<(), QueryError> {
        reg.resources
            .write_consume(self.guard)
            .map_err(QueryError::LockableError)?;

        Ok(())
    }
}

impl Query<EngineState> for Stage {
    fn prepare(reg: &mut EngineState) -> Result<Self, QueryError> {
        Ok(reg.stage.clone())
    }

    fn consume(self, _: &mut EngineState) -> Result<(), QueryError> {
        Ok(())
    }
}

impl<T> Query<T> for () {
    fn prepare(_: &mut T) -> Result<Self, QueryError> {
        Ok(())
    }

    fn consume(self, _: &mut T) -> Result<(), QueryError> {
        Ok(())
    }
}

macro_rules! impl_tuple_query {
    ($($t:ident),+) => {
        #[allow(non_snake_case)]
        impl<Reg,$($t: Query<Reg>),+> Query<Reg> for ($($t,)+) {
            fn prepare(reg: &mut Reg) -> Result<Self, QueryError> {
                let ($($t,)+) = ($($t::prepare(reg),)+);

                match ($($t,)+) {
                    ($(Ok($t),)+) => {Ok(($($t,)+))}
                    ($($t,)+)=> {
                        $(if let Ok($t) = $t {
                            $t::consume($t, reg)?;
                        })+ ;
                        Err(QueryError::TupleError)
                    }
                }
            }

            fn consume(self, reg: &mut Reg) -> Result<(), QueryError> {
                let ($($t,)+) = self;
                $($t.consume(reg).map_err(|_|QueryError::TupleError)?;)+
                Ok(())
            }
        }
    };
}

impl_all!(impl_tuple_query);
