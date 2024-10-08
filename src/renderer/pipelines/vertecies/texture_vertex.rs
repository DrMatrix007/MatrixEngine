use wgpu::{Buffer, VertexAttribute, VertexBufferLayout};

use super::MatrixVertexBufferable;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TextureVertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

pub struct TextureVertexBuffers<'a> {
    pub vertex_buffer: &'a Buffer,
    pub index_buffer: &'a Buffer,
}

impl MatrixVertexBufferable for TextureVertex {
    const ATTRS: &'static [VertexAttribute] =
        &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

    fn vertex_buffer_layout() -> VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: Self::ATTRS,
        }
    }

    type Buffer<'a> = TextureVertexBuffers<'a>;

    fn setup_pass(pass: &mut wgpu::RenderPass<'_>, index: u32, buffer: Self::Buffer<'_>) {
        pass.set_vertex_buffer(index, buffer.vertex_buffer.slice(..));
        pass.set_index_buffer(buffer.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
    }
}
