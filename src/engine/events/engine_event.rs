use std::time::Instant;

use crate::engine::{scenes::entities::Entity, systems::SystemControlFlow};

#[derive(Clone, Copy, Debug)]
pub enum EngineEvent {
    SystemDone(Entity, SystemControlFlow),
    UpdateDeltaTime { frame_start: Instant },
}
