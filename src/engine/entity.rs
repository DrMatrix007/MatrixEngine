type EntityId = usize;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Entity {
    id: EntityId,
}

impl Entity {
    pub fn from_id(id: EntityId) -> Self {
        Self { id }
    }

    pub fn id(&self) -> EntityId {
        self.id
    }
}

pub struct EntityCreator {
    counter: usize,
}

impl Default for EntityCreator {
    fn default() -> Self {
        Self::new()
    }
}

impl EntityCreator {
    pub fn new() -> Self {
        Self { counter: 0 }
    }

    pub fn create_entity(&mut self) -> Entity {
        let entity = Entity::from_id(self.counter);
        self.counter += 1;
        entity
    }
}
