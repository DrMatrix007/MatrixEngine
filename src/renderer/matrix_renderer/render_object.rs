use std::any::TypeId;

use crate::engine::scenes::components::Component;

use crate::renderer::pipelines::{
    buffers::{Vertex, VertexBuffer},
    instance_manager::VertexStructure,
};

use super::renderer_system::DeviceQueue;

pub struct RenderObject {
    buffer: Box<dyn VertexStructure<Vertex> + Sync + Send>,
    texture_name: String,
}

impl RenderObject {
    pub fn new(
        structure: impl VertexStructure<Vertex> + Send + Sync,
        texture_name: String,
    ) -> Self {
        Self {
            buffer: Box::new(structure),
            texture_name,
        }
    }

    pub fn texture_name(&self) -> &str {
        &self.texture_name
    }
    pub fn structure_type_id(&self) -> TypeId {
        self.buffer.type_id()
    }
    pub fn create_buffer(&self, device: &DeviceQueue) -> VertexBuffer<Vertex> {
        self.buffer.create_buffer(device)
    }
}

impl Component for RenderObject {}
