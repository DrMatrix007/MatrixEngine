use crate::math::matrices::{Matrix4, Vector3};

pub struct Transform {
    pub mat: Matrix4<f32>,
}

impl Component for Transform {}

impl Transform {
    pub fn identity() -> Self {
        Self {
            mat: Matrix4::identity(),
        }
    }

    pub fn apply_position_diff(&mut self, position: Vector3<f32>) {
        self.mat = &self.mat
            * Matrix4::from([
                [1., 0.0, 0.0, 0.0],
                [0.0, 1., 0.0, 0.0],
                [0.0, 0.0, 1., 0.0],
                [*position.x(), *position.y(), *position.z(), 1.],
            ]);
    }
    pub fn with_position_diff(mut self, position: Vector3<f32>) -> Self {
        self.apply_position_diff(position);
        self
    }
    pub fn apply_rotation(&mut self, rotation: Vector3<f32>) {
        self.mat = &self.mat * rotation.euler_into_rotation_matrix4();
    }

    pub fn with_rotation(mut self, rotation: Vector3<f32>) -> Self {
        self.apply_rotation(rotation);
        self
    }

    pub fn apply_scale(&mut self, scale: Vector3<f32>) {
        self.mat = &self.mat
            * Matrix4::from([
                [*scale.x(), 0.0, 0.0, 0.0],
                [0.0, *scale.y(), 0.0, 0.0],
                [0.0, 0.0, *scale.z(), 0.0],
                [0., 0., 0., 1.],
            ]);
    }

    pub fn with_scale(mut self, scale: Vector3<f32>) -> Self {
        self.apply_scale(scale);
        self
    }
}
