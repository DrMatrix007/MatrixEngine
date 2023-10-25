use std::f32::consts::PI;

use bytemuck::Zeroable;
use wgpu::BufferUsages;

use crate::renderer::pipelines::buffers::{BufferContainer, Vertex, VertexBuffer};

use super::VertexStructure;

pub struct Circle<const STEPS: u16>;

impl<const STEPS: u16> VertexStructure<Vertex> for Circle<STEPS> {
    fn create_buffer(
        &self,
        device: &crate::renderer::matrix_renderer::renderer_system::DeviceQueue,
    ) -> crate::renderer::pipelines::buffers::VertexBuffer<Vertex> {
        let mut vertecies = Vec::with_capacity(STEPS as usize + 1);
        let mut indexes = Vec::<u16>::new();

        vertecies.resize(STEPS as usize + 1, Zeroable::zeroed());
        vertecies[0] = Vertex {
            position: [0., 0., 0.],
            texture_pos: [0.5, 0.5],
        };
        for i in 0..STEPS {
            let current = (2. * i as f32 / STEPS as f32) * PI;
            let current_tex = current - PI;
            vertecies[i as usize + 1] = Vertex {
                position: [current.cos() / 2., current.sin() / 2., 0.],
                texture_pos: [0.5 + current_tex.cos() / 2., 0.5 + current_tex.sin() / 2.],
            }
        }
        for i in 2..(STEPS+1) {
            indexes.push(i);
            indexes.push(i - 1);
            indexes.push(0);
        }
        indexes.push(1);
        indexes.push(vertecies.len() as u16 - 1);
        indexes.push(0);
        if indexes.len() % 2 != 0 {
            indexes.push(0);
        }
        VertexBuffer::new(
            BufferContainer::create_buffer(
                &vertecies,
                device,
                BufferUsages::COPY_DST | BufferUsages::VERTEX,
                false,
            ),
            Some(BufferContainer::create_buffer(
                &indexes,
                device,
                BufferUsages::INDEX | BufferUsages::COPY_DST,
                false,
            )),
        )
    }
}
