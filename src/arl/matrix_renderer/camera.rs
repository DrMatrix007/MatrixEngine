use bytemuck::{Pod, Zeroable};
use cgmath::{Matrix4, PerspectiveFov, Point3, SquareMatrix, Vector3};
use num_traits::{Float, cast};
use wgpu::{BufferUsages, ShaderStages};

use crate::arl::{
    bind_groups::{BindGroupLayoutEntry, BindGroupable},
    buffers::Buffer,
    device_queue::DeviceQueue,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CameraID {
    Defualt,
}

pub struct Camera {
    pub pos: Point3<f32>,
    pub direction: Vector3<f32>,
    pub up: Vector3<f32>,
    pub perspective: PerspectiveFov<f32>,

    raw: CameraRaw,
}

impl Camera {
    pub fn new(
        pos: Point3<f32>,
        direction: Vector3<f32>,
        up: Vector3<f32>,
        perspective: PerspectiveFov<f32>,
    ) -> Self {
        Self {
            pos,
            direction,
            up,
            perspective,
            raw: CameraRaw {
                proj: Matrix4::identity().into(),
            },
        }
    }

    pub fn update_raw(&mut self) {
        // self.raw.proj = Matrix::identity()
        let mat = OPENGL_TO_WGPU_MATRIX
            * Matrix4::from(self.perspective)
            * Matrix4::look_to_rh(self.pos, self.direction, self.up);

        self.raw.proj = mat.into();
    }

    pub fn raw(&self) -> CameraRaw {
        self.raw
    }
}

#[repr(C)]
#[derive(Debug, Pod, Zeroable, Clone, Copy)]
pub struct CameraRaw {
    pub proj: [[f32; 4]; 4],
}

pub struct CameraUniform {
    buffer: Buffer<CameraRaw>,
}

impl CameraUniform {
    pub fn write(&self, raw: &CameraRaw) {
        self.buffer.write(core::slice::from_ref(raw));
    }
}

impl BindGroupable for CameraUniform {
    type BindGroupID = CameraID;

    fn new(_: &Self::BindGroupID, device_queue: &DeviceQueue) -> Self {
        Self {
            buffer: Buffer::new(
                "camera buffer",
                &[CameraRaw {
                    proj: [[0.0; 4]; 4],
                }],
                BufferUsages::UNIFORM | BufferUsages::COPY_DST,
                device_queue.clone(),
            ),
        }
    }

    fn label(&self) -> String {
        "camera uniform bind group".to_string()
    }

    fn layout_label() -> String {
        "camera uniform layout".to_string()
    }

    fn get_layout_entries() -> &'static [crate::arl::bind_groups::BindGroupLayoutEntry] {
        &[BindGroupLayoutEntry {
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
            visibility: ShaderStages::VERTEX,
        }]
    }

    fn get_group_entries(&self) -> impl AsRef<[wgpu::BindingResource<'_>]> {
        [self.buffer.raw().as_entire_binding()]
    }

    fn id(&self) -> Self::BindGroupID {
        todo!()
    }
}

pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::from_cols(
    cgmath::Vector4::new(1.0, 0.0, 0.0, 0.0),
    cgmath::Vector4::new(0.0, 1.0, 0.0, 0.0),
    cgmath::Vector4::new(0.0, 0.0, 0.5, 0.0),
    cgmath::Vector4::new(0.0, 0.0, 0.5, 1.0),
);
