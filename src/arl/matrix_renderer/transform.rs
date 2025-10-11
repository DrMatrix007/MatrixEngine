use bytemuck::{Pod, Zeroable};

use crate::{
    arl::vertex::instantiable::Instantiable,
    math::{
        matrix::{Matrix, Matrix4, RowVector},
        quaternions::Quaternion,
    },
};

pub struct Transform {
    pos: RowVector<3, f32>,
    quat: RowVector<4, f32>,
    scale: RowVector<3, f32>,

    raw: TransformRaw,
}

impl Transform {
    pub fn new(pos: RowVector<3, f32>, quat: RowVector<4, f32>, scale: RowVector<3, f32>) -> Self {
        Self {
            pos,
            quat,
            scale,
            raw: TransformRaw::new(),
        }
    }

    pub fn update_raw(&mut self) {
        let mut trans = Matrix4::<f32>::identity();
        trans[(3, 0)] = self.pos[0];
        trans[(3, 1)] = self.pos[1];
        trans[(3, 2)] = self.pos[2];

        let rot = self.quat.to_rot_matrix();

        let scale = Matrix4::from_fn(|n, m| {
            if n == m {
                if n < 3 { self.scale[n] } else { 1.0 }
            } else {
                0.0
            }
        });

        let res = trans;

        self.raw.raw = res;
    }

    pub fn raw(&self) -> &TransformRaw {
        &self.raw
    }
}

#[repr(C)]
#[derive(Pod, Zeroable, Debug, Clone, Copy)]
pub struct TransformRaw {
    pub raw: Matrix<4, 4, f32>,
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
            raw: Matrix::zeros(),
        }
    }
}

impl Default for TransformRaw {
    fn default() -> Self {
        Self::new()
    }
}
