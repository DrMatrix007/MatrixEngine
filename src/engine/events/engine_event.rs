use crate::engine::{scenes::entities::Entity, systems::SystemControlFlow};

pub enum EngineEvent {
    SystemDone(Entity, SystemControlFlow),
}
