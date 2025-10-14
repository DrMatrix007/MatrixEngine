use bytemuck::{Pod, Zeroable};
use cgmath::{Matrix4, Quaternion, SquareMatrix, Vector3};

use crate::arl::vertex::instantiable::Instantiable;

#[repr(C)]
pub struct Transform {
    pos: Vector3<f32>,
    quat: Quaternion<f32>,
    scale: Vector3<f32>,
    raw: TransformRaw,
}

impl Transform {
    pub fn new(pos: Vector3<f32>, quat: Quaternion<f32>, scale: Vector3<f32>) -> Self {
        Self {
            pos,
            quat,
            scale,
            raw: TransformRaw::new(),
        }
    }

    pub fn update_raw(&mut self) {
        let trans = Matrix4::from_translation(self.pos);
        // // let _rot = self.quat.to_rot_matrix();

        // // let _scale = Matrix4::from_fn(|n, m| {
        // //     if n == m {
        // //         if n < 3 { self.scale[n] } else { 1.0 }
        // //     } else {
        // //         0.0
        // //     }
        // // });

        // // let res = trans;

        self.raw.raw = trans.into();
    }

    pub fn raw(&self) -> &TransformRaw {
        &self.raw
    }
}

#[repr(C)]
#[derive(Pod, Zeroable, Debug, Clone, Copy)]
pub struct TransformRaw {
    pub raw: [[f32; 4]; 4],
}

impl Instantiable for TransformRaw {
    fn desc() -> impl AsRef<[wgpu::VertexFormat]> {
        [
            wgpu::VertexFormat::Float32x4,
            wgpu::VertexFormat::Float32x4,
            wgpu::VertexFormat::Float32x4,
            wgpu::VertexFormat::Float32x4,
        ]
    }
}

impl TransformRaw {
    pub fn new() -> Self {
        Self {
            raw: Matrix4::identity().into(),
        }
    }
}

impl Default for TransformRaw {
    fn default() -> Self {
        Self::new()
    }
}
