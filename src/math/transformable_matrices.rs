use std::ops::{Add, AddAssign, Mul};

use num_traits::{cast, Float, One, Zero};

use super::{
    matrices::{Matrix3, Matrix4, Vector3},
    vectors::{Vector, Vector3D},
};

pub trait TransformMatrix {
    type Sub;

    fn look_to_rh(eye: &Self::Sub, dir: &Self::Sub, up: &Self::Sub) -> Self;
    fn look_to_lh(eye: &Self::Sub, dir: &Self::Sub, up: &Self::Sub) -> Self;

    fn look_at_rh(eye: &Self::Sub, center: &Self::Sub, up: &Self::Sub) -> Self;
    fn look_at_lh(eye: &Self::Sub, center: &Self::Sub, up: &Self::Sub) -> Self;
}

impl<T: Mul<Output = T> + Add<Output = T> + AddAssign<T> + Zero + Float> TransformMatrix
    for Matrix4<T>
{
    type Sub = Vector3<T>;

    fn look_to_rh(eye: &Self::Sub, dir: &Self::Sub, up: &Self::Sub) -> Self {
        let f = dir.normalized();
        let s = f.cross(up).normalized();
        let u = s.cross(&f);

        Matrix4::from([
            [*s.x(), *u.x(), -*f.x(), T::zero()],
            [*s.y(), *u.y(), -*f.y(), T::zero()],
            [*s.z(), *u.z(), -*f.z(), T::zero()],
            [-eye.dot(&s), -eye.dot(&u), eye.dot(&f), T::one()],
        ])
    }

    fn look_to_lh(eye: &Self::Sub, dir: &Self::Sub, up: &Self::Sub) -> Self {
        Self::look_to_rh(eye, &-dir, up)
    }

    fn look_at_rh(eye: &Self::Sub, center: &Self::Sub, up: &Self::Sub) -> Self {
        Self::look_to_rh(eye, &(center - eye), up)
    }

    fn look_at_lh(eye: &Self::Sub, center: &Self::Sub, up: &Self::Sub) -> Self {
        Self::look_to_lh(eye, &(center - eye), up)
    }
}
pub struct Prespective<T> {
    pub fovy_rad: T,
    pub aspect: T,
    pub near: T,
    pub far: T,
}

impl<T: Zero + Float> From<&'_ Prespective<T>> for Matrix4<T> {
    fn from(value: &'_ Prespective<T>) -> Self {
        assert!(value.near < value.far);
        assert!(!value.aspect.is_zero());

        let two: T = cast(2.0_f32).expect("the value 2 is needed");

        let f = T::one() / (value.fovy_rad / two).tan();
        Matrix4::from([
            [f / value.aspect, T::zero(), T::zero(), T::zero()],
            [T::zero(), f, T::zero(), T::zero()],
            [
                T::zero(),
                T::zero(),
                (value.far + value.near) / (value.near - value.far),
                -T::one(),
            ],
            [
                T::zero(),
                T::zero(),
                (two * value.far * value.near) / (value.near - value.far),
                T::zero(),
            ],
        ])
    }
}

impl<T: Zero + Float> From<Prespective<T>> for Matrix4<T> {
    fn from(value: Prespective<T>) -> Self {
        (&value).into()
    }
}

impl<T: Float + Zero + One> Matrix4<T> {
    pub fn rotate_x(angle: T) -> Self {
        Self::from([
            [T::one(), T::zero(), T::zero(), T::zero()],
            [T::zero(), angle.cos(), -angle.sin(), T::zero()],
            [T::zero(), angle.sin(), angle.cos(), T::zero()],
            [T::zero(), T::zero(), T::zero(), T::one()],
        ])
    }
    pub fn rotate_y(angle: T) -> Self {
        Self::from([
            [angle.cos(), T::zero(), angle.sin(), T::zero()],
            [T::zero(), T::one(), T::zero(), T::zero()],
            [-angle.sin(), T::zero(), angle.cos(), T::zero()],
            [T::zero(), T::zero(), T::zero(), T::one()],
        ])
    }
    pub fn rotate_z(angle: T) -> Self {
        Self::from([
            [angle.cos(), -angle.sin(), T::zero(), T::zero()],
            [angle.sin(), angle.cos(), T::zero(), T::zero()],
            [T::zero(), T::zero(), T::one(), T::zero()],
            [T::zero(), T::zero(), T::zero(), T::one()],
        ])
    }
}

impl<T: Float + Zero + One> Matrix3<T> {
    pub fn rotate_x(angle: T) -> Self {
        Self::from([
            [T::one(), T::zero(), T::zero()],
            [T::zero(), angle.cos(), -angle.sin()],
            [T::zero(), angle.sin(), angle.cos()],
        ])
    }
    pub fn rotate_y(angle: T) -> Self {
        Self::from([
            [angle.cos(), T::zero(), angle.sin()],
            [T::zero(), T::one(), T::zero()],
            [-angle.sin(), T::zero(), angle.cos()],
        ])
    }
    pub fn rotate_z(angle: T) -> Self {
        Self::from([
            [angle.cos(), -angle.sin(), T::zero()],
            [angle.sin(), angle.cos(), T::zero()],
            [T::zero(), T::zero(), T::one()],
        ])
    }
}

impl Vector3<f32> {
    pub fn euler_into_rotation_matrix3(&self) -> Matrix3<f32> {
        Matrix3::rotate_y(*self.y())
        * Matrix3::rotate_x(*self.x())
        * Matrix3::rotate_x(*self.z())
    }
    pub fn euler_into_rotation_matrix4(&self) -> Matrix4<f32> {
        Matrix4::rotate_y(*self.y())
        * Matrix4::rotate_x(*self.x())
        * Matrix4::rotate_x(*self.z())
    }
}
