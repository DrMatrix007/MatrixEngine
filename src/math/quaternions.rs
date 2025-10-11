use std::ops::Index;

use num_traits::Float;

use crate::math::matrix::{ColVector, Matrix, RowVector};

pub trait Quaternion<T: Float>: Index<usize, Output = T> {
    fn to_rot_matrix(&self) -> Matrix<4, 4, T> {
        let [a, b, c, d] = [self[0], self[1], self[2], self[3]];

        Matrix::new([[a, -b, -c, -d], [b, a, -d, c], [c, d, a, -b], [d, -c, b, a]])
    }
}

impl<T: Float> Quaternion<T> for RowVector<4, T> {}
impl<T: Float> Quaternion<T> for ColVector<4, T> {}
