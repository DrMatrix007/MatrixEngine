use crate::{
    engine::{
        component::{Component, ComponentRegistry},
        entity::Entity,
    },
    lockable::LockableError,
};

pub trait ComponentGroup {
    fn add_self(self, entity: &Entity, reg: &mut ComponentRegistry) -> Result<(), LockableError>;
}
impl<T: Component> ComponentGroup for (T,) {
    fn add_self(self, entity: &Entity, reg: &mut ComponentRegistry) -> Result<(), LockableError> {
        let mut guard = reg.write_components::<T>()?;

        guard.insert(entity, self.0);

        reg.write_components_consume(guard)?;

        Ok(())
    }
}
impl<A: ComponentGroup, B: ComponentGroup> ComponentGroup for (A, B) {
    fn add_self(self, entity: &Entity, reg: &mut ComponentRegistry) -> Result<(), LockableError> {
        let (a, b) = self;
        a.add_self(entity, reg)?;
        b.add_self(entity, reg)?;

        Ok(())
    }
}

pub struct AddEntityCommand<Components: ComponentGroup> {
    components: Components,
}

impl<Components: ComponentGroup> AddEntityCommand<Components> {
    pub fn with<C: Component>(self, comp: C) -> AddEntityCommand<((C,), Components)> {
        AddEntityCommand {
            components: ((comp,), self.components),
        }
    }
}

impl<C: Component> AddEntityCommand<(C,)> {
    pub fn new(component: C) -> Self {
        Self {
            components: (component,),
        }
    }
}
