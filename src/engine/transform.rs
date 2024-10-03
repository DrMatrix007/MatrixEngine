use wgpu::{vertex_attr_array, Buffer, VertexBufferLayout};

use crate::{
    math::matrix::{Matrix4, Vector3, Vector4},
    renderer::pipelines::vertecies::MatrixVertexBufferable,
};

pub struct Transform {
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub scale: Vector3<f32>,

    pub raw: TransformRaw,
}

impl Transform {
    pub fn new(position: Vector3<f32>, rotation: Vector3<f32>, scale: Vector3<f32>) -> Self {
        let mut r = Self {
            position,
            rotation,
            scale,
            raw: TransformRaw { mat: [[0.0; 4]; 4] },
        };
        r.update_raw();
        r
    }
    pub fn new_position(position: Vector3<f32>) -> Self {
        Self::new(position, Vector3::zeros(), Vector3::build_with(|| 1.))
    }
    pub fn update_raw(&mut self) {
        self.raw = TransformRaw {
            mat: (&Matrix4::from_position(&self.position)
                * &Matrix4::from_quaternion(&Vector4::from_euler_to_quaternion(&self.rotation)))
                .into_storage(),
        }
    }
    pub fn raw(&self) -> &TransformRaw {
        &self.raw
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TransformRaw {
    pub mat: [[f32; 4]; 4],
}

impl MatrixVertexBufferable for TransformRaw {
    const ATTRS: &[wgpu::VertexAttribute] =
        &vertex_attr_array![2=>Float32x4,3=>Float32x4,4=>Float32x4,5=>Float32x4];

    fn vertex_buffer_layout() -> wgpu::VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: core::mem::size_of::<TransformRaw>() as _,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: Self::ATTRS,
        }
    }

    type Buffer<'a> = &'a Buffer;

    fn setup_pass(pass: &mut wgpu::RenderPass<'_>, index: u32, buffer: Self::Buffer<'_>) {
        pass.set_vertex_buffer(index, buffer.slice(..));
    }
}
