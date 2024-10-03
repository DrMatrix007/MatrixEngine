use std::fmt::Display;

use num_traits::Float;

use super::{
    matrix::{Matrix4, Vector3},
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
}
