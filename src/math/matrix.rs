use std::{
    iter::Sum,
    ops::{Add, Index, IndexMut, Mul, Sub},
};

use bytemuck::{Pod, Zeroable};
use num_traits::Num;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Matrix<const N: usize, const M: usize, T: Num> {
    matrix: [[T; N]; M],
}

unsafe impl<const N: usize, const M: usize, T: Num + Pod + Copy> Pod for Matrix<N, M, T> {}
unsafe impl<const N: usize, const M: usize, T: Num + Pod + Zeroable> Zeroable for Matrix<N, M, T> {}

impl<const N: usize, const M: usize, T: Num> Matrix<N, M, T> {
    const IS_VECTOR: bool = (N == 1) ^ (M == 1); // XOR: exactly one of them is 1
    const IS_ROW_VECTOR: bool = (M == 1);
    const IS_COL_VECTOR: bool = (N == 1);

    pub fn new(matrix: [[T; N]; M]) -> Self {
        Self { matrix }
    }

    pub fn from_fn(mut f: impl FnMut(usize, usize) -> T) -> Self {
        Self {
            matrix: core::array::from_fn(|m| core::array::from_fn(|n| f(m, n))),
        }
    }

    pub fn zero() -> Self {
        Self::from_fn(|_, _| T::zero())
    }
    pub fn identity() -> Self {
        Self::from_fn(|m, n| if m == n { T::one() } else { T::zero() })
    }

    pub fn raw(&self) -> &[[T; N]; M] {
        &self.matrix
    }
}

impl<const N: usize, const M: usize, T: Num + Copy> Add for Matrix<N, M, T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let result =
            std::array::from_fn(|i| std::array::from_fn(|j| self.matrix[i][j] + rhs.matrix[i][j]));

        Matrix::new(result)
    }
}

impl<const N: usize, const M: usize, T: Num + Copy> Sub for Matrix<N, M, T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let result =
            std::array::from_fn(|i| std::array::from_fn(|j| self.matrix[i][j] - rhs.matrix[i][j]));

        Matrix::new(result)
    }
}

impl<const M: usize, const K: usize, const N: usize, T: Num + Copy + Sum> Mul<Matrix<K, M, T>>
    for Matrix<N, K, T>
{
    type Output = Matrix<N, M, T>;

    fn mul(self, rhs: Matrix<K, M, T>) -> Self::Output {
        let result = std::array::from_fn(|i| {
            std::array::from_fn(|j| (0..K).map(|k| self.matrix[i][k] * rhs.matrix[k][j]).sum())
        });
        Matrix::new(result)
    }
}

pub type RowVector<const N: usize, T> = Matrix<N, 1, T>;
pub type ColVector<const N: usize, T> = Matrix<1, N, T>;
pub type SquareMatrix<const N: usize, T> = Matrix<N, N, T>;

pub type Matrix2<T> = SquareMatrix<2, T>;
pub type Matrix3<T> = SquareMatrix<3, T>;
pub type Matrix4<T> = SquareMatrix<4, T>;

impl<const N: usize, const M: usize, T: Num> Index<usize> for Matrix<N, M, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(
            Self::IS_VECTOR,
            "Indexing with usize only valid for vectors"
        );

        if Self::IS_ROW_VECTOR {
            &self.matrix[0][index]
        } else if Self::IS_COL_VECTOR {
            &self.matrix[index][0]
        } else {
            panic!("Indexing with usize not supported for general matrices");
        }
    }
}

impl<const N: usize, const M: usize, T: Num> IndexMut<usize> for Matrix<N, M, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(
            Self::IS_VECTOR,
            "Indexing with usize only valid for vectors"
        );

        if Self::IS_ROW_VECTOR {
            &mut self.matrix[0][index]
        } else if Self::IS_COL_VECTOR {
            &mut self.matrix[index][0]
        } else {
            panic!("Indexing with usize not supported for general matrices");
        }
    }
}

impl<const N: usize, const M: usize, T: Num> Index<(usize, usize)> for Matrix<N, M, T> {
    type Output = T;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.matrix[row][col]
    }
}

impl<const N: usize, const M: usize, T: Num> IndexMut<(usize, usize)> for Matrix<N, M, T> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.matrix[row][col]
    }
}
