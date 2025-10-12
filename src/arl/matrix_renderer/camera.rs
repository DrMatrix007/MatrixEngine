use bytemuck::{Pod, Zeroable};
use num_traits::{Float, cast, float::FloatCore};
use wgpu::{BufferUsages, ShaderStages};

use crate::{
    arl::{
        bind_groups::{BindGroupLayoutEntry, BindGroupable},
        buffers::Buffer,
        device_queue::DeviceQueue,
    },
    math::{
        matrix::{ColVector, Matrix, Matrix4},
        vector::{CrossableVector, Vector},
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CameraID {
    Defualt,
}

pub struct Camera {
    pub pos: ColVector<3, f32>,
    pub direction: ColVector<3, f32>,
    pub up: ColVector<3, f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,

    raw: CameraRaw,
}

impl Camera {
    pub fn new(
        pos: ColVector<3, f32>,
        direction: ColVector<3, f32>,
        up: ColVector<3, f32>,
        aspect: f32,
        fovy: f32,
        znear: f32,
        zfar: f32,
    ) -> Self {
        Self {
            pos,
            direction,
            up,
            aspect,
            fovy,
            znear,
            zfar,
            raw: CameraRaw {
                proj: Matrix::zeros(),
            },
        }
    }

    pub fn update_raw(&mut self) {
        // self.raw.proj = Matrix::identity()
        self.raw.proj = OPENGL_TO_WGPU_MATRIX
            * Matrix4::prespective(&self.fovy, &self.aspect, &self.znear, &self.zfar)
            * Matrix4::look_to_rh(&self.pos, &self.direction, &self.up)
    }

    pub fn raw(&self) -> CameraRaw {
        self.raw
    }
}

#[repr(C)]
#[derive(Debug, Pod, Zeroable, Clone, Copy)]
pub struct CameraRaw {
    pub proj: Matrix4<f32>,
}

pub trait CameraLookableMatrix {
    type Position;
    type Scalar;

    fn look_to_rh(eye: &Self::Position, dir: &Self::Position, up: &Self::Position) -> Self;
    fn prespective(
        fovy: &Self::Scalar,
        aspect: &Self::Scalar,
        near: &Self::Scalar,
        far: &Self::Scalar,
    ) -> Self;
}

impl<T: Float> CameraLookableMatrix for Matrix4<T> {
    type Position = ColVector<3, T>;

    type Scalar = T;

    fn look_to_rh(eye: &Self::Position, dir: &Self::Position, up: &Self::Position) -> Self {
        let forward = dir.normalized().unwrap();
        let right = forward.cross(up).normalized().unwrap();
        let new_up = right.cross(&forward);

        Matrix4::new([
            [right[0], new_up[0], -forward[0], T::zero()],
            [right[1], new_up[1], -forward[1], T::zero()],
            [right[2], new_up[2], -forward[2], T::zero()],
            [
                -eye.dot(&right),
                -eye.dot(&new_up),
                eye.dot(&forward),
                T::one(),
            ],
        ])
    }

    fn prespective(
        fovy: &Self::Scalar,
        aspect: &Self::Scalar,
        near: &Self::Scalar,
        far: &Self::Scalar,
    ) -> Self {
        let two: T = cast(2).unwrap();
        let fovy = T::one() / T::tan(*fovy / two);

        Matrix4::new([
            [fovy / *aspect, T::zero(), T::zero(), T::zero()],
            [T::zero(), fovy, T::zero(), T::zero()],
            [
                T::zero(),
                T::zero(),
                (*far + *near) / (*near - *far),
                -T::one(),
            ],
            [
                T::zero(),
                T::zero(),
                (two * *far * *near) / (*near - *far),
                T::zero(),
            ],
        ])
    }
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
                    proj: Matrix::identity(),
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

pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new([
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 0.5, 0.0],
    [0.0, 0.0, 0.5, 1.0],
]);
