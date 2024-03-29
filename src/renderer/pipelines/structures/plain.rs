use wgpu::BufferUsages;

use crate::renderer::{
    matrix_renderer::renderer_system::DeviceQueue,
    pipelines::buffers::{BufferContainer, Vertex, VertexBuffer},
};

use super::VertexStructure;

pub struct Plain;

impl VertexStructure<Vertex> for Plain {
    fn create_buffer(&self, device: &DeviceQueue) -> VertexBuffer<Vertex> {
        VertexBuffer::new(
            BufferContainer::<Vertex>::create_buffer(
                &Self::VERTICES,
                device,
                BufferUsages::COPY_DST | BufferUsages::VERTEX,
                false,
            ),
            Some(BufferContainer::<u16>::create_buffer(
                &Self::INDEXES,
                device,
                BufferUsages::INDEX | BufferUsages::COPY_DST,
                false,
            )),
        )
    }
}

impl Plain {
    const VERTICES: &[Vertex] = &[
        Vertex {
            position: [-0.5, 0.5, 0.0],
            texture_pos: [0., 0.],
        },
        Vertex {
            position: [0.5, 0.5, 0.0],
            texture_pos: [1.0, 0.0],
        },
        Vertex {
            position: [0.5, -0.5, 0.0],
            texture_pos: [1.0, 1.0],
        },
        Vertex {
            position: [-0.5, -0.5, 0.0],
            texture_pos: [0.0, 1.0],
        },
    ];
    const INDEXES: &[u16] = &[0, 1, 2, 2,3,0];
}
