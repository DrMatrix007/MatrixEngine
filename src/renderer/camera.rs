use std::{borrow::BorrowMut, sync::Arc};

use lazy_static::lazy_static;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroupEntry, BindGroupLayoutEntry, Buffer, BufferUsages, ShaderStages,
};

use crate::math::matrix::{Matrix4, Vector3};

use super::pipelines::{bind_groups::bind::MatrixBindable, device_queue::DeviceQueue};

pub struct Camera {
    pub eye: Vector3<f32>,
    pub target: Vector3<f32>,
    pub up: Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Camera {
    pub fn build_view_projection_matrix(&self) -> Matrix4<f32> {
        let view = Matrix4::look_to_rh(&self.eye, &self.target, &self.up);

        let proj = Matrix4::perspective(self.fovy, self.aspect, self.znear, self.zfar);

        let opengl_to_wgpu = Matrix4::from_storage([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 0.5, 0.5],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        &(&opengl_to_wgpu * &view) * &proj
        // &view * &proj
    }
}

pub(crate) struct CameraUniform {
    view_proj: Arc<Buffer>,
}

impl CameraUniform {
    pub(crate) fn new(device_queue: &DeviceQueue) -> Self {
        Self {
            view_proj: Arc::new(
                device_queue
                    .device()
                    .create_buffer_init(&BufferInitDescriptor {
                        label: Some("camera uniform buffer"),
                        usage: BufferUsages::COPY_DST | BufferUsages::UNIFORM,
                        contents: bytemuck::cast_slice(&Matrix4::<f32>::one().into_storage()),
                    }),
            ),
        }
    }

    pub(crate) fn update_view_proj(&mut self, device_queue: &DeviceQueue, camera: &Camera) {
        device_queue.queue().write_buffer(
            &self.view_proj,
            0,
            bytemuck::cast_slice(&camera.build_view_projection_matrix().into_storage()),
        );
    }
}

impl MatrixBindable for CameraUniform {
    fn bind_layout_entry(index: u32) -> wgpu::BindGroupLayoutEntry {
        BindGroupLayoutEntry {
            binding: index,
            visibility: ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }
    }

    fn bind_entry(&self, binding: u32) -> wgpu::BindGroupEntry {
        BindGroupEntry {
            binding,
            resource: self.view_proj.as_entire_binding(),
        }
    }
}
