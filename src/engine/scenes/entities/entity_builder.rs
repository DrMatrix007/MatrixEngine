

use crate::engine::scenes::components::{component_registry::ComponentRegistry, Component};

use super::Entity;

pub struct EntityBuilder<'a>(Entity, &'a mut ComponentRegistry);

impl<'a> EntityBuilder<'a> {
    pub fn new(reg: &'a mut ComponentRegistry) -> EntityBuilder<'a> {
        Self(Entity::new(), reg)
    }
    pub fn add<C: Component + 'static>(self, component: C) -> Result<EntityBuilder<'a>, C> {
        match self.1.try_add_component(self.0, component) {
            Ok(_) => Ok(self),
            Err(c) => Err(c),
        }
    }
}
