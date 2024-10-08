use std::{
    fmt::Display,
    marker::PhantomData,
    ops::{Index, IndexMut},
};

use num_traits::Float;

use super::{matrix_storage::MatrixStoragable, number::Number};

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
    pub fn from_storage(storage: Storage) -> Self {
        Self {
            storage,
            marker: PhantomData,
        }
    }

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
    pub fn into_storage(self) -> Storage {
        self.storage
    }

    pub fn iter(&self) -> impl Iterator<Item = ((usize, usize), &T)> {
        self.storage.iter_pos()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = ((usize, usize), &mut T)> {
        self.storage.iter_pos_mut()
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
    use std::ops::{AddAssign, SubAssign};

    use crate::{
        impl_ops_binary,
        math::{matrix_storage::MatrixStoragable, number::Number},
    };

    use super::Matrix;

    // impl<T: Number, const M: usize, const N: usize, Storage: MatrixStoragable<T, M, N>> Neg
    //     for &Matrix<T, M, N, Storage>
    // {
    //     type Output = Matrix<T, M, N, Storage>;

    //     fn neg(self) -> Self::Output {
    //         Self::Output::build_with_pos(|i, j| -self[(i, j)].clone())
    //     }
    // }
    // #[opimps::impl_uni_ops(Neg)]
    // fn neg<T: Number, const M: usize, const N: usize, Storage: MatrixStoragable<T, M, N>>(
    //     m: &Matrix<T, M, N, Storage>,
    // ) -> Matrix<T, M, N, Storage> {
    //     Matrix::build_with_pos(|i, j| -m[(i, j)].clone())
    // }
    // gen_ops!{
    //     <T, const M: usize, const N: usize, Storage: MatrixStoragable<T, M, N>>;
    //     types Matrix<T,M,N,Storage>, Matrix<T,M,N,Storage> => Matrix<T,M,N,Storage>;
    //     for + call |m: &Matrix<T, M, N, Storage>,m2: &Matrix<T, M, N, Storage>| {add(m,m2)};
    //     (where T:Number)

    //     for - call |m: &Matrix<T, M, N, Storage>,m2: &Matrix<T, M, N, Storage>| {add(m,m2)};
    //     (where T:Number)

    //     where T:Number
    // };

    impl_ops_binary!(+ |a1:?Matrix<T, M, N, Storage>,a2:?Matrix<T, M, N, Storage>| -> Matrix<T, M, N, Storage> {
                            Matrix::build_with_pos(|i, j| a1[(i, j)].clone()+a2[(i,j)].clone())
                        } generic(<T: Number, const M: usize, const N: usize, Storage: MatrixStoragable<T, M, N>>));

    impl_ops_binary!(* |a1:?Matrix<T, M, N, Storage1>,a2:?Matrix<T, N, P, Storage2>| -> Matrix<T, M, P, Storage1::SelfWith<M,P>> {
                            Matrix::build_with_pos(|m,p|{
                                (0..N).map(|n|a1[(m,n)].clone()*a2[(n,p)].clone()).fold(T::zero(), |a,b|a+b)
                            })
                        } generic(<T: Number, const M: usize, const N: usize,const P:usize, Storage1: MatrixStoragable<T, M, N>,Storage2:MatrixStoragable<T,N,P>>));
    impl_ops_binary!(/ |a1:?Matrix<T, M, N, Storage>,a2: ?T| -> Matrix<T, M, N, Storage> {
                            Matrix::build_with_pos(|i, j| a1[(i, j)].clone() / a2.clone())
                        } generic(<T: Number, const M: usize, const N: usize, Storage: MatrixStoragable<T, M, N>>));

    impl_ops_binary!(* |a1:?Matrix<T, M, N, Storage>,a2: ?T| -> Matrix<T, M, N, Storage> {
                            Matrix::build_with_pos(|i, j| a1[(i, j)].clone() * a2.clone())
                        } generic(<T: Number, const M: usize, const N: usize, Storage: MatrixStoragable<T, M, N>>));
    
    impl<
            T: Number + AddAssign<T>,
            const M: usize,
            const N: usize,
            Storage: MatrixStoragable<T, M, N>,
        > AddAssign for Matrix<T, M, N, Storage>
    {
        fn add_assign(&mut self, other: Self) {
            self.iter_mut().for_each(|(i, val)| {
                *val += other[i].clone();
            });
        }
    }

    impl<
            T: Number + SubAssign<T>,
            const M: usize,
            const N: usize,
            Storage: MatrixStoragable<T, M, N>,
        > SubAssign for Matrix<T, M, N, Storage>
    {
        fn sub_assign(&mut self, other: Self) {
            self.iter_mut().for_each(|(i, val)| {
                *val -= other[i].clone();
            });
        }
    }
}

pub type Vector<T, const M: usize> = Matrix<T, M, 1>;
pub type Vector2<T> = Vector<T, 2>;
pub type Vector3<T> = Vector<T, 3>;
pub type Vector4<T> = Vector<T, 4>;

pub type Matrix4<T> = Matrix<T, 4, 4>;

impl<T: Number + Float, const M: usize> Vector<T, M> {
    pub fn normalized(&self) -> Self {
        self / (0..M)
            .map(|x| self[(x, 0)] * self[(x, 0)])
            .fold(T::zero(), |x, y| x + y)
            .sqrt()
    }

    pub fn dot(&self, rhs: &Self) -> T {
        (0..M)
            .map(|m| self[(m, 0)] * rhs[(m, 0)])
            .fold(T::zero(), |x, y| x + y)
    }
}

impl<T: Number> Vector3<T> {
    pub fn cross(&self, other: &Vector3<T>) -> Vector3<T> {
        Vector3::from_storage([
            [(self[(1, 0)].clone() * other[(2, 0)].clone())
                - (self[(2, 0)].clone() * other[(1, 0)].clone())], // y * z - z * y
            [(self[(2, 0)].clone() * other[(0, 0)].clone())
                - (self[(0, 0)].clone() * other[(2, 0)].clone())], // z * x - x * z
            [(self[(0, 0)].clone() * other[(1, 0)].clone())
                - (self[(1, 0)].clone() * other[(0, 0)].clone())], // x * y - y * x
        ])
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

        let result = &matrix_a * &matrix_b;

        for i in 0..3 {
            for j in 0..3 {
                assert_eq!(result[(i, j)], expected[(i, j)]);
            }
        }
    }
}
