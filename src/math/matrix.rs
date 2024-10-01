use std::{
    fmt::Display,
    marker::PhantomData,
    ops::{Index, IndexMut},
};

use super::{matrix_storage::MatrixStoragable, number::Number};

#[derive(Debug, Clone)]
pub struct Matrix<
    T: Number,
    const M: usize,
    const N: usize,
    Storage: MatrixStoragable<T, M, N> = [[T; N]; M],
> {
    storage: Storage,
    marker: PhantomData<T>,
}

impl<T: Number, const M: usize, const N: usize, Storage: MatrixStoragable<T, M, N>>
    IndexMut<(usize, usize)> for Matrix<T, M, N, Storage>
{
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self.storage.get_mut(index)
    }
}

impl<T: Number, const M: usize, const N: usize, Storage: MatrixStoragable<T, M, N>>
    Index<(usize, usize)> for Matrix<T, M, N, Storage>
{
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.storage.get(index)
    }
}

impl<T: Number, const M: usize, const N: usize, Storage: MatrixStoragable<T, M, N>>
    Matrix<T, M, N, Storage>
{
    pub fn zeros() -> Self {
        Self {
            storage: Storage::zeros(),
            marker: PhantomData,
        }
    }

    pub fn one() -> Self {
        Self::build_with_pos(|y, x| if y == x { T::one() } else { T::zero() })
    }

    pub fn build_with(f: impl FnMut() -> T) -> Self {
        Self {
            storage: Storage::build_with(f),
            marker: PhantomData,
        }
    }

    pub fn build_with_pos(f: impl FnMut(usize, usize) -> T) -> Self {
        Self {
            storage: Storage::build_with_pos(f),
            marker: PhantomData,
        }
    }
}

impl<T: Number + Display, const M: usize, const N: usize, Storage: MatrixStoragable<T, M, N>>
    Display for Matrix<T, M, N, Storage>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for m in 0..M {
            write!(f, "{}[", if m != 0 { " " } else { "[" })?;
            for n in 0..N {
                self[(m, n)].fmt(f)?;
                if n != N - 1 {
                    write!(f, ", ")?;
                }
            }
            write!(f, "]{}", if m == M - 1 { "]" } else { ",\n" })?;
        }
        Ok(())
    }
}

mod ops {
    use std::ops::{Add, Mul, Sub};

    use crate::math::{matrix_storage::MatrixStoragable, number::Number};

    use super::Matrix;

    impl<T: Number, const M: usize, const N: usize, Storage: MatrixStoragable<T, M, N>> Add
        for Matrix<T, M, N, Storage>
    {
        type Output = Self;

        fn add(self, other: Self) -> Self::Output {
            Self::build_with_pos(|i, j| self[(i, j)].clone() + other[(i, j)].clone())
        }
    }

    // Implement subtraction
    impl<T: Number, const M: usize, const N: usize, Storage: MatrixStoragable<T, M, N>> Sub
        for Matrix<T, M, N, Storage>
    {
        type Output = Self;

        fn sub(self, other: Self) -> Self::Output {
            Self::build_with_pos(|i, j| self[(i, j)].clone() - other[(i, j)].clone())
        }
    }

    impl<
            T: Number,
            const M: usize,
            const N: usize,
            const P: usize,
            StorageA: MatrixStoragable<T, M, N>,
            StorageB: MatrixStoragable<T, N, P>,
        > Mul<Matrix<T, N, P, StorageB>> for Matrix<T, M, N, StorageA>
    {
        type Output = Matrix<T, M, P>;

        fn mul(self, rhs: Matrix<T, N, P, StorageB>) -> Self::Output {
            Matrix::build_with_pos(|i, j| {
                (0..N)
                    .map(|k| self[(i, k)].clone() * rhs[(k, j)].clone())
                    .fold(T::zero(), |a, b| a + b)
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Matrix;

    #[test]
    fn test_matrix_multiplication() {

        let mut matrix_a = Matrix::<i32, 3, 3>::zeros();
        let mut matrix_b = Matrix::<i32, 3, 3>::zeros();

        // [1, 2, 3]
        // [4, 5, 6]
        // [7, 8, 9]
        for i in 0..3 {
            for j in 0..3 {
                matrix_a[(i, j)] = (i * 3 + j + 1) as i32;
            }
        }

        // [9, 8, 7]
        // [6, 5, 4]
        // [3, 2, 1]
        for i in 0..3 {
            for j in 0..3 {
                matrix_b[(i, j)] = (9 - (i * 3 + j)) as i32;
            }
        }

        // [ 30,  24,  18]
        // [ 84,  69,  54]
        // [138, 114,  90]
        let mut expected = Matrix::<i32, 3, 3>::zeros();
        expected[(0, 0)] = 30;
        expected[(0, 1)] = 24;
        expected[(0, 2)] = 18;
        expected[(1, 0)] = 84;
        expected[(1, 1)] = 69;
        expected[(1, 2)] = 54;
        expected[(2, 0)] = 138;
        expected[(2, 1)] = 114;
        expected[(2, 2)] = 90;

        let result = matrix_a * matrix_b;

        for i in 0..3 {
            for j in 0..3 {
                assert_eq!(result[(i, j)], expected[(i, j)]);
            }
        }
    }
}
