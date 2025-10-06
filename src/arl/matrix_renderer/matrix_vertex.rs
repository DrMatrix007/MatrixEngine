use crate::arl::vertex_buffer::VertexBuffer;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MatrixVertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl VertexBuffer for MatrixVertex {
    fn desc() -> (wgpu::VertexStepMode, Vec<wgpu::VertexFormat>) {
        (
            wgpu::VertexStepMode::Vertex,
            vec![wgpu::VertexFormat::Float32x3, wgpu::VertexFormat::Float32x3],
        )
    }
}
