pub mod texture_vertex;

use wgpu::{VertexAttribute, VertexBufferLayout};

pub trait Vertexable {
    const ATTRS: &[VertexAttribute];
    fn vertex_buffer_layout() -> VertexBufferLayout<'static>;
}
