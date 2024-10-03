use std::fmt::Display;

use num_traits::Float;

use super::{
    matrix::{Matrix4, Vector3, Vector4},
    number::Number,
};

impl<T: Number + Float + Display> Matrix4<T> {
    pub fn look_to_rh(eye: &Vector3<T>, dir: &Vector3<T>, up: &Vector3<T>) -> Matrix4<T> {
        let f = dir.normalized();
        let s = f.cross(up).normalized();
        let u = s.cross(&f);
        let x = Matrix4::from_storage([
            [*s.x(), *u.x(), -*f.x(), T::zero()],
            [*s.y(), *u.y(), -*f.y(), T::zero()],
            [*s.z(), *u.z(), -*f.z(), T::zero()],
            [-eye.dot(&s), -eye.dot(&u), eye.dot(&f), T::one()],
        ]);
        x
    }

    pub fn perspective(fovy_rad: T, aspect: T, znear: T, zfar: T) -> Matrix4<T> {
        let two: T = num_traits::cast(2.0).unwrap();
        let f = T::one() / (fovy_rad / two).tan();

        let c0r0 = f / aspect;
        let c0r1 = T::zero();
        let c0r2 = T::zero();
        let c0r3 = T::zero();

        let c1r0 = T::zero();
        let c1r1 = f;
        let c1r2 = T::zero();
        let c1r3 = T::zero();

        let c2r0 = T::zero();
        let c2r1 = T::zero();
        let c2r2 = (zfar + znear) / (znear - zfar);
        let c2r3 = -T::one();

        let c3r0 = T::zero();
        let c3r1 = T::zero();
        let c3r2 = (two * zfar * znear) / (znear - zfar);
        let c3r3 = T::zero();

        Matrix4::from_storage([
            [c0r0, c0r1, c0r2, c0r3],
            [c1r0, c1r1, c1r2, c1r3],
            [c2r0, c2r1, c2r2, c2r3],
            [c3r0, c3r1, c3r2, c3r3],
        ])
    }

    pub fn from_position(position: &Vector3<T>) -> Self {
        Self::from_storage([
            [T::one(), T::zero(), T::zero(), T::zero()],
            [T::zero(), T::one(), T::zero(), T::zero()],
            [T::zero(), T::zero(), T::one(), T::zero()],
            [*position.x(), *position.y(), *position.z(), T::one()],
        ])
    }

    pub fn from_quaternion(quat: &Vector4<T>) -> Self {
        let x2 = *quat.x() + *quat.x();
        let y2 = *quat.y() + *quat.y();
        let z2 = *quat.z() + *quat.z();

        let xx2 = x2 * *quat.x();
        let xy2 = x2 * *quat.y();
        let xz2 = x2 * *quat.z();

        let yy2 = y2 * *quat.y();
        let yz2 = y2 * *quat.z();
        let zz2 = z2 * *quat.z();

        let sy2 = y2 * *quat.w(); // `w()` is the scalar part of the quaternion
        let sz2 = z2 * *quat.w();
        let sx2 = x2 * *quat.w();

        // Use from_storage to construct the matrix from the array
        Matrix4::from_storage([
            [T::one() - yy2 - zz2, xy2 + sz2, xz2 - sy2, T::zero()],
            [xy2 - sz2, T::one() - xx2 - zz2, yz2 + sx2, T::zero()],
            [xz2 + sy2, yz2 - sx2, T::one() - xx2 - yy2, T::zero()],
            [T::zero(), T::zero(), T::zero(), T::one()],
        ])
    }
}

impl<T: Number + Float> Vector4<T> {
    pub fn from_euler_to_quaternion(src: &Vector3<T>) -> Self {
        let half = num_traits::cast(1.0f64).unwrap();
        let (s_x, c_x) = (*src.x() * half).sin_cos();
        let (s_y, c_y) = (*src.y() * half).sin_cos();
        let (s_z, c_z) = (*src.z() * half).sin_cos();

        Vector4::new(
            -s_x * s_y * s_z + c_x * c_y * c_z,
            s_x * c_y * c_z + s_y * s_z * c_x,
            -s_x * s_z * c_y + s_y * c_x * c_z,
            s_x * s_y * c_z + s_z * c_x * c_y,
        )
    }
}
