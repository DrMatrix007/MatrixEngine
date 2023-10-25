use wgpu::BufferUsages;

use crate::renderer::{
    matrix_renderer::renderer_system::DeviceQueue,
    pipelines::buffers::{BufferContainer, Vertex, VertexBuffer},
};

use super::VertexStructure;
pub struct Cube;

impl VertexStructure<Vertex> for Cube {
    fn create_buffer(&self, device: &DeviceQueue) -> VertexBuffer<Vertex> {
        VertexBuffer::new(
            BufferContainer::<Vertex>::create_buffer(
                &Self::VERTICES,
                device,
                BufferUsages::COPY_DST | BufferUsages::VERTEX,
                false,
            ),
            Some(BufferContainer::<u16>::create_buffer(
                &Self::INDICES,
                device,
                BufferUsages::INDEX | BufferUsages::COPY_DST,
                false,
            )),
        )
    }
}

impl Cube {
    const VERTICES: &[Vertex] = &[
        Vertex {
            position: [-0.5, -0.5, -0.5],
            texture_pos: [0.0, 0.0],
        }, // A 0
        Vertex {
            position: [0.5, -0.5, -0.5],
            texture_pos: [1.0, 0.0],
        }, // B 1
        Vertex {
            position: [0.5, 0.5, -0.5],
            texture_pos: [1.0, 1.0],
        }, // C 2
        Vertex {
            position: [-0.5, 0.5, -0.5],
            texture_pos: [0.0, 1.0],
        }, // D 3
        Vertex {
            position: [-0.5, -0.5, 0.5],
            texture_pos: [0.0, 0.0],
        }, // E 4
        Vertex {
            position: [0.5, -0.5, 0.5],
            texture_pos: [1.0, 0.0],
        }, // F 5
        Vertex {
            position: [0.5, 0.5, 0.5],
            texture_pos: [1.0, 1.0],
        }, // G 6
        Vertex {
            position: [-0.5, 0.5, 0.5],
            texture_pos: [0.0, 1.0],
        }, // H 7
        Vertex {
            position: [-0.5, 0.5, -0.5],
            texture_pos: [0.0, 0.0],
        }, // D 8
        Vertex {
            position: [-0.5, -0.5, -0.5],
            texture_pos: [1.0, 0.0],
        }, // A 9
        Vertex {
            position: [-0.5, -0.5, 0.5],
            texture_pos: [1.0, 1.0],
        }, // E 10
        Vertex {
            position: [-0.5, 0.5, 0.5],
            texture_pos: [0.0, 1.0],
        }, // H 11
        Vertex {
            position: [0.5, -0.5, -0.5],
            texture_pos: [0.0, 0.0],
        }, // B 12
        Vertex {
            position: [0.5, 0.5, -0.5],
            texture_pos: [1.0, 0.0],
        }, // C 13
        Vertex {
            position: [0.5, 0.5, 0.5],
            texture_pos: [1.0, 1.0],
        }, // G 14
        Vertex {
            position: [0.5, -0.5, 0.5],
            texture_pos: [0.0, 1.0],
        }, // F 15
        Vertex {
            position: [-0.5, -0.5, -0.5],
            texture_pos: [0.0, 0.0],
        }, // A 16
        Vertex {
            position: [0.5, -0.5, -0.5],
            texture_pos: [1.0, 0.0],
        }, // B 17
        Vertex {
            position: [0.5, -0.5, 0.5],
            texture_pos: [1.0, 1.0],
        }, // F 18
        Vertex {
            position: [-0.5, -0.5, 0.5],
            texture_pos: [0.0, 1.0],
        }, // E 19
        Vertex {
            position: [0.5, 0.5, -0.5],
            texture_pos: [0.0, 0.0],
        }, // C 20
        Vertex {
            position: [-0.5, 0.5, -0.5],
            texture_pos: [1.0, 0.0],
        }, // D 21
        Vertex {
            position: [-0.5, 0.5, 0.5],
            texture_pos: [1.0, 1.0],
        }, // H 22
        Vertex {
            position: [0.5, 0.5, 0.5],
            texture_pos: [0.0, 1.0],
        }, // G 23
    ];

    const INDICES: &[u16] = &[
        // front and back
        0, 3, 2, 2, 1, 0, 4, 5, 6, 6, 7, 4, // left and right
        11, 8, 9, 9, 10, 11, 12, 13, 14, 14, 15, 12, // bottom and top
        16, 17, 18, 18, 19, 16, 20, 21, 22, 22, 23, 20,
    ];
}
