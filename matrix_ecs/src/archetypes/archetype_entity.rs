use crate::entities::Entity;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ArchetypeEntity(Entity);

impl From<Entity> for ArchetypeEntity {
    fn from(value: Entity) -> Self {
        Self(value)
    }
}
