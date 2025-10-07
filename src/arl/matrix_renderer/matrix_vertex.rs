use crate::arl::vertex::Vertex;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MatrixVertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex for MatrixVertex {
    fn desc() -> (wgpu::VertexStepMode, Vec<wgpu::VertexFormat>) {
        (
            wgpu::VertexStepMode::Vertex,
            vec![wgpu::VertexFormat::Float32x3, wgpu::VertexFormat::Float32x3],
        )
    }
}
