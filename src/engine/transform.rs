use wgpu::{vertex_attr_array, Buffer, VertexBufferLayout};

use crate::{math::matrix::Vector3, renderer::pipelines::vertecies::MatrixVertexBufferable};

#[derive(Debug)]
pub struct Transform {
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub scale: Vector3<f32>,
}

impl Transform {
    pub fn new(position: Vector3<f32>, rotation: Vector3<f32>, scale: Vector3<f32>) -> Self {
        Self {
            position,
            rotation,
            scale,
        }
    }
    pub fn new_position(position: Vector3<f32>) -> Self {
        Self::new(position, Vector3::zeros(), Vector3::build_with(|| 1.))
    }
    pub fn raw(&self) -> TransformRaw {
        TransformRaw {
            mat: [
                [
                    *self.position.x(),
                    *self.position.y(),
                    *self.position.z(),
                    0., // Padding or unused element
                ],
                [
                    *self.rotation.x(),
                    *self.rotation.y(),
                    *self.rotation.z(),
                    0., // Padding or unused element
                ],
                [
                    *self.scale.x(),
                    *self.scale.y(),
                    *self.scale.z(),
                    0., // Padding or unused element
                ],
                [0., 0., 0., 1.], // Last row for homogeneous coordinates
            ], // other: [0.; 3],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TransformRaw {
    // saves position, rotation and scale in different row
    pub mat: [[f32; 4]; 4],
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TransformMat {
    pub(crate) mat: [[f32; 4]; 4],
}

impl MatrixVertexBufferable for TransformMat {
    const ATTRS: &'static [wgpu::VertexAttribute] =
        &vertex_attr_array![2=>Float32x4,3=>Float32x4,4=>Float32x4,5=>Float32x4];

    fn vertex_buffer_layout() -> wgpu::VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: core::mem::size_of::<TransformMat>() as _,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: Self::ATTRS,
        }
    }

    type Buffer<'a> = &'a Buffer;

    fn setup_pass(pass: &mut wgpu::RenderPass<'_>, index: u32, buffer: Self::Buffer<'_>) {
        pass.set_vertex_buffer(index, buffer.slice(..));
    }
}
