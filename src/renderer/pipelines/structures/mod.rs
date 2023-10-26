use std::any::Any;

use crate::renderer::matrix_renderer::renderer_system::DeviceQueue;

use super::buffers::{Bufferable, VertexBuffer};

pub mod circle;
pub mod cube;
pub mod plain;
pub mod icosphere;

pub trait VertexStructure<Vertex: Bufferable>: Any {
    fn create_buffer(&self, device: &DeviceQueue) -> VertexBuffer<Vertex>;
}
