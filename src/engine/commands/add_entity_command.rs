use crate::{
    engine::{
        EngineState,
        commands::{Command, CommandError},
        component::{Component, ComponentRegistry},
        entity::Entity,
    },
    lockable::LockableError,
};

pub trait ComponentGroup: Send {
    fn add_self(self, entity: &Entity, reg: &mut ComponentRegistry) -> Result<(), LockableError>;
}

impl ComponentGroup for () {
    fn add_self(self, _: &Entity, _: &mut ComponentRegistry) -> Result<(), LockableError> {
        Ok(())
    }
}



impl<T: Component> ComponentGroup for (T,) {
    fn add_self(self, entity: &Entity, reg: &mut ComponentRegistry) -> Result<(), LockableError> {
        let mut guard = reg.write::<T>()?;

        guard.insert(entity, self.0);

        reg.write_consume(guard)?;

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
    components: Option<Components>,
}

impl<Components: ComponentGroup> AddEntityCommand<Components> {
    pub fn with<C: Component>(self, comp: C) -> Option<AddEntityCommand<((C,), Components)>> {
        Some(AddEntityCommand {
            components: Some(((comp,), self.components?)),
        })
    }
}

impl AddEntityCommand<()> {
    pub fn new() -> Self {
        Self {
            components: Some(()),
        }
    }
}

impl Default for AddEntityCommand<()> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C: ComponentGroup> Command for AddEntityCommand<C> {
    type Args = EngineState;
    fn run(&mut self, data: &mut EngineState) -> Result<(), CommandError> {
        let entity = data.entity_creator.create_entity();
        let component = self.components.take();
        match component {
            Some(component)=> component
            .add_self(&entity, &mut data.registry.components)
            .map_err(CommandError::LockableError)?,
            None => return Err(CommandError::UnknownError)
        };
        
        Ok(())
    }
}
