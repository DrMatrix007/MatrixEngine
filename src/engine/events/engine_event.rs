use crate::engine::{scenes::entities::Entity, systems::SystemControlFlow};

#[derive(Clone, Copy,Debug)]
pub enum EngineEvent {
    SystemDone(Entity, SystemControlFlow),
}
