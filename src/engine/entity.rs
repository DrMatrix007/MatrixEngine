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
