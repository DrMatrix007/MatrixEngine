use wgpu::{VertexAttribute, VertexBufferLayout};

use super::Vertexable;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct TextureVertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

impl Vertexable for TextureVertex {
    const ATTRS: &[VertexAttribute] = &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

    fn vertex_buffer_layout() -> VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: Self::ATTRS,
        }
    }
}
