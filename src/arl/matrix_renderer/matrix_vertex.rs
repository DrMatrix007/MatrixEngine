use crate::arl::vertex::vertexable::Vertexable;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MatrixVertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl Vertexable for MatrixVertex {
    fn desc() -> impl AsRef<[wgpu::VertexFormat]> {
        [wgpu::VertexFormat::Float32x3, wgpu::VertexFormat::Float32x3]
    }
}
