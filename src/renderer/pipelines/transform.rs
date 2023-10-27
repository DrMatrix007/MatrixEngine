use crate::engine::scenes::components::transform::Transform;
use bytemuck::{Pod, Zeroable};
use wgpu::{BufferAddress, VertexAttribute};

use crate::math::matrices::Matrix4;

use super::buffers::Bufferable;

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
pub struct InstanceTransform {
    data: [[f32; 4]; 4],
}

impl From<&Transform> for InstanceTransform {
    fn from(value: &Transform) -> Self {
        InstanceTransform {
            data: value.mat.clone().into(),
        }
    }
}

impl From<Matrix4<f32>> for InstanceTransform {
    fn from(value: Matrix4<f32>) -> Self {
        Self { data: value.into() }
    }
}

impl Default for InstanceTransform {
    fn default() -> Self {
        Self {
            data: Matrix4::identity().into(),
        }
    }
}

impl InstanceTransform {
    const ATTRS: &[VertexAttribute] = &[
        wgpu::VertexAttribute {
            offset: 0,
            shader_location: 5,
            format: wgpu::VertexFormat::Float32x4,
        },
        wgpu::VertexAttribute {
            offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
            shader_location: 6,
            format: wgpu::VertexFormat::Float32x4,
        },
        wgpu::VertexAttribute {
            offset: std::mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
            shader_location: 7,
            format: wgpu::VertexFormat::Float32x4,
        },
        wgpu::VertexAttribute {
            offset: std::mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
            shader_location: 8,
            format: wgpu::VertexFormat::Float32x4,
        },
    ];
}

impl Bufferable for InstanceTransform {
    fn describe<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<InstanceTransform>() as BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: Self::ATTRS,
        }
    }
}
