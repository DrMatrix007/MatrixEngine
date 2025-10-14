use crate::arl::vertex::vertexable::Vertexable;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MatrixVertex {
    pub pos: [f32; 3],
    pub tex_pos: [f32; 2],
}

impl Vertexable for MatrixVertex {
    fn desc() -> impl AsRef<[wgpu::VertexFormat]> {
        [wgpu::VertexFormat::Float32x3, wgpu::VertexFormat::Float32x2]
    }
}
