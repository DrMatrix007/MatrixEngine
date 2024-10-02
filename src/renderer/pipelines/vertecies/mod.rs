pub mod texture_vertex;

use wgpu::{VertexAttribute, VertexBufferLayout};

pub trait Vertexable: 'static {
    const ATTRS: &[VertexAttribute];
    fn vertex_buffer_layout() -> VertexBufferLayout<'static>;
}
