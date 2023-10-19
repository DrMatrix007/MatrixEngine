use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Div, Index, IndexMut, Mul, Neg, Sub},
};

use num_traits::{One, Zero};

#[derive(Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Matrix<T, const N: usize, const M: usize>([[T; N]; M]);

impl<T: 'static, const N: usize, const M: usize> IntoIterator for Matrix<T, N, M> {
    fn into_iter(self) -> Self::IntoIter {
        Box::new(
            self.0
                .into_iter()
                .enumerate()
                .flat_map(|(m, l)| l.into_iter().enumerate().map(move |(n, val)| ((m, n), val))),
        )
    }

    type Item = ((usize, usize), T);

    type IntoIter = Box<dyn Iterator<Item = ((usize, usize), T)>>;
}

impl<T: Clone, const N: usize, const M: usize> Clone for Matrix<T, N, M> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: Neg<Output = T> + Clone, const N: usize, const M: usize> Neg for &Matrix<T, N, M> {
    type Output = Matrix<T, N, M>;

    fn neg(self) -> Self::Output {
        self.map(|_, x| -x)
    }
}

impl<T: Neg<Output = T> + Clone, const N: usize, const M: usize> Neg for Matrix<T, N, M> {
    type Output = Matrix<T, N, M>;

    fn neg(self) -> Self::Output {
        -&self
    }
}

// impl<const N: usize, const M: usize, T> FromIterator<T, Matrix<N, 1>> for Matrix<T, N, M> {
//     fn from_iter<T: IntoIterator<Item = Matrix<T,N, 1>>>(iter: T) -> Self {
//         Matrix(iter.into_iter().flat_map(|x| x.0).collect())
//     }
// }

impl<const N: usize, const M: usize, T: Display> Display for Matrix<T, N, M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;

        for (pos, x) in self.iter() {
            if pos.0 == 0 {
                if pos.1 != 0 {
                    write!(f, " ")?;
                }
                write!(f, "[")?
            }
            write!(f, "{}", x)?;
            if pos.0 == N - 1 {
                if pos.1 == M - 1 {
                    write!(f, "]")?;
                } else {
                    writeln!(f, "],")?;
                }
            } else {
                write!(f, ",")?;
            }
        }

        write!(f, "]")
    }
}

impl<const N: usize, const M: usize, T: Default> Default for Matrix<T, N, M> {
    fn default() -> Self {
        Self::generate(Default::default)
    }
}

impl<const N: usize, const M: usize, T> Matrix<T, N, M> {
    pub fn iter(&self) -> impl Iterator<Item = ((usize, usize), &T)> {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(m, l)| l.iter().enumerate().map(move |(n, val)| ((m, n), val)))
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = ((usize, usize), &mut T)> {
        self.0
            .iter_mut()
            .enumerate()
            .flat_map(|(m, l)| l.iter_mut().enumerate().map(move |(n, val)| ((m, n), val)))
    }

    pub fn trasnpose(&self) -> Matrix<T, M, N>
    where
        T: Default + Clone,
    {
        let mut ans = Matrix::default();
        for (pos, i) in ans.iter_mut() {
            *i = self[(pos.1, pos.0)].clone();
        }
        ans
    }
    pub fn generate(mut f: impl FnMut() -> T) -> Self {
        Self([[(); N]; M].map(|x| x.map(|_| f())))
    }

    pub fn element_wise_product(&self, x: &Matrix<T, N, M>) -> Matrix<T, N, M>
    where
        T: Default + Mul<T, Output = T> + Clone,
    {
        let mut ans = Matrix::default();

        for (pos, i) in ans.iter_mut() {
            *i = x[pos].clone() * self[pos].clone();
        }
        ans
    }

    pub fn map(&self, mut f: impl FnMut((usize, usize), T) -> T) -> Matrix<T, N, M>
    where
        T: Clone,
    {
        let mut ans = self.clone();
        for (pos, val) in ans.iter_mut() {
            *val = f(pos, val.clone());
        }
        ans
    }
    pub fn max(&self) -> T
    where
        T: std::cmp::Ord + Clone,
    {
        self.iter()
            .map(|(_, y)| y)
            .fold(self[(0, 0)].clone(), |x, y| T::max(x, y.clone()))
    }
    pub fn sub_matrices_vertically(&self) -> impl Iterator<Item = Matrix<T, N, 1>> + '_
    where
        T: Clone + Default,
    {
        self.0.iter().map(|x| Matrix::from([x.clone()]))
    }

    // pub fn sub(&self, i: usize) -> Matrix<T, N, 1>
    // where
    //     T: Clone,
    // {
    //     if i < M {
    //         Matrix(self.0.iter().skip(i * N).take(N).cloned().collect())
    //     } else {
    //         panic!("access bounds out of range");
    //     }
    // }
    // pub fn set_sub(&mut self, i: usize, m: &Matrix<T, N, 1>) {
    //     for x in 0..N {
    //         self[(x, i)] = m[(x, 0)];
    //     }
    // }
    pub fn add_matrix(&self, rhs: &Self) -> Self
    where
        T: Add<T, Output = T> + Clone,
    {
        self.map(|x, y| y + rhs[x].clone())
    }
    pub fn sub_matrix(&self, rhs: &Self) -> Self
    where
        T: Sub<T, Output = T> + Clone,
    {
        self.map(|x, y| y - rhs[x].clone())
    }
    pub fn mul_matrix<const K: usize>(&self, rhs: &Matrix<T, M, K>) -> Matrix<T, N, K>
    where
        T: Mul<T, Output = T> + AddAssign<T> + Default + Clone,
    {
        let mut ans = Matrix::default();

        for (pos, x) in ans.iter_mut() {
            *x = Default::default();
            for m in 0..M {
                *x += self[(m, pos.1)].clone() * rhs[(pos.0, m)].clone();
            }
        }
        ans
    }
    pub fn mul_element_wise(&self, m: &Matrix<T, N, M>) -> Self
    where
        T: Mul<T, Output = T> + Default + Clone,
    {
        self.map(|pos, x| x * m[pos].clone())
    }
    pub fn div_element_wise(&self, m: &Matrix<T, N, M>) -> Self
    where
        T: Div<T, Output = T> + Default + Clone,
    {
        self.map(|pos, x| x / m[pos].clone())
    }

    pub fn zeros() -> Self
    where
        T: Zero,
    {
        Self::generate(|| T::zero())
    }
    pub fn ones() -> Self
    where
        T: One,
    {
        Self::generate(|| T::one())
    }
    pub fn identity() -> Self
    where
        T: Zero + One,
    {
        let mut ans = Self::zeros();
        for i in 0..N.min(M) {
            ans[(i, i)] = T::one();
        }

        ans
    }

    pub fn into_arrays(self) -> [[T; N]; M] {
        self.into()
    }
}

pub trait IntoMatrix<T, const N: usize, const M: usize> {
    fn into_matrix(self) -> Matrix<T, N, M>;
}
impl<T, const N: usize, const M: usize> IntoMatrix<T, N, M> for [[T; N]; M] {
    fn into_matrix(self) -> Matrix<T, N, M> {
        Matrix(self)
    }
}

impl<const N: usize, const M: usize, T> Index<(usize, usize)> for Matrix<T, N, M> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.0[index.0][index.1]
    }
}

impl<const N: usize, const M: usize, T> IndexMut<(usize, usize)> for Matrix<T, N, M> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.0[index.0][index.1]
    }
}

impl<const N: usize, T> Index<usize> for Matrix<T, N, 1> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[0][index]
    }
}

impl<const N: usize, T> IndexMut<usize> for Matrix<T, N, 1> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[0][index]
    }
}

impl<T, const N: usize, const M: usize> From<[[T; N]; M]> for Matrix<T, N, M> {
    fn from(val: [[T; N]; M]) -> Self {
        Matrix(val)
    }
}

impl<T, const N: usize, const M: usize> From<Matrix<T, N, M>> for [[T; N]; M] {
    fn from(val: Matrix<T, N, M>) -> Self {
        val.0
    }
}

#[test]
fn test_into() {
    let m = Matrix::from([[1, 2], [2, 3]]);
    println!("{:?}", m.into_arrays());
}

impl<T: Clone, const N: usize, const M: usize> From<&'_ Matrix<T, N, M>> for [[T; N]; M]
where
    T: Debug + Clone,
{
    fn from(val: &Matrix<T, N, M>) -> Self {
        (0..M)
            .map(|m| {
                (0..N)
                    .map(|n| val[(n, m)].clone())
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }
}

// impl<T: Into<f64> + Clone, const N: usize> ToMatrix<N, 1> for [T; N] {
//     fn to_matrix(self) -> Matrix<N, 1> {
//         Matrix(self.concat().into_iter().map(|x| x.into()).collect())
//     }
// }

impl<T, const N: usize, const M: usize> Matrix<T, N, M> {
    pub fn clone_cast<A>(&self) -> Matrix<A, N, M>
    where
        T: Into<A> + Clone,
    {
        Matrix(self.0.clone().map(|x| x.map(|x| x.into())))
    }
    pub fn into_cast<A>(self) -> Matrix<A, N, M>
    where
        T: Into<A>,
    {
        Matrix(self.0.map(|x| x.map(|x| x.into())))
    }
}

mod add {
    use std::ops::{Add, AddAssign};

    use super::Matrix;
    impl<const N: usize, const M: usize, T> Add<&Matrix<T, N, M>> for &Matrix<T, N, M>
    where
        T: Add<T, Output = T> + Clone,
    {
        type Output = Matrix<T, N, M>;

        fn add(self, rhs: &Matrix<T, N, M>) -> Self::Output {
            self.add_matrix(rhs)
        }
    }
    impl<const N: usize, const M: usize, T> Add<Matrix<T, N, M>> for Matrix<T, N, M>
    where
        T: Add<T, Output = T> + Clone,
    {
        type Output = Matrix<T, N, M>;

        fn add(self, rhs: Matrix<T, N, M>) -> Self::Output {
            self.add_matrix(&rhs)
        }
    }
    impl<const N: usize, const M: usize, T> Add<&Matrix<T, N, M>> for Matrix<T, N, M>
    where
        T: Add<T, Output = T> + Clone,
    {
        type Output = Matrix<T, N, M>;

        fn add(self, rhs: &Matrix<T, N, M>) -> Self::Output {
            (self).add_matrix(rhs)
        }
    }
    impl<const N: usize, const M: usize, T> Add<Matrix<T, N, M>> for &Matrix<T, N, M>
    where
        T: Add<T, Output = T> + Default + Clone,
    {
        type Output = Matrix<T, N, M>;

        fn add(self, rhs: Matrix<T, N, M>) -> Self::Output {
            self.add_matrix(&rhs)
        }
    }

    impl<const N: usize, const M: usize, T> AddAssign<&Matrix<T, N, M>> for Matrix<T, N, M>
    where
        T: Add<T, Output = T> + Default + Clone,
    {
        fn add_assign(&mut self, rhs: &Matrix<T, N, M>) {
            *self = self.add_matrix(rhs);
        }
    }
    impl<const N: usize, const M: usize, T> AddAssign<Matrix<T, N, M>> for Matrix<T, N, M>
    where
        T: Add<T, Output = T> + Default + Clone,
    {
        fn add_assign(&mut self, rhs: Matrix<T, N, M>) {
            *self += &rhs;
        }
    }
}
mod sub {
    use std::ops::Sub;

    use super::Matrix;

    impl<const N: usize, const M: usize, T> Sub<&Matrix<T, N, M>> for &Matrix<T, N, M>
    where
        T: Sub<Output = T> + Clone,
    {
        type Output = Matrix<T, N, M>;

        fn sub(self, rhs: &Matrix<T, N, M>) -> Self::Output {
            self.sub_matrix(rhs)
        }
    }
    impl<const N: usize, const M: usize, T> Sub<Matrix<T, N, M>> for Matrix<T, N, M>
    where
        T: Sub<Output = T> + Clone,
    {
        type Output = Matrix<T, N, M>;

        fn sub(self, rhs: Matrix<T, N, M>) -> Self::Output {
            self.sub_matrix(&rhs)
        }
    }
    impl<const N: usize, const M: usize, T> Sub<&Matrix<T, N, M>> for Matrix<T, N, M>
    where
        T: Sub<Output = T> + Clone,
    {
        type Output = Matrix<T, N, M>;

        fn sub(self, rhs: &Matrix<T, N, M>) -> Self::Output {
            self.sub_matrix(rhs)
        }
    }
    impl<const N: usize, const M: usize, T> Sub<Matrix<T, N, M>> for &Matrix<T, N, M>
    where
        T: Sub<Output = T> + Clone,
    {
        type Output = Matrix<T, N, M>;

        fn sub(self, rhs: Matrix<T, N, M>) -> Self::Output {
            self.sub_matrix(&rhs)
        }
    }
}
mod mul {
    use std::ops::{AddAssign, Mul};

    use super::Matrix;

    impl<const N: usize, const M: usize, const K: usize, T> Mul<&Matrix<T, M, K>> for &Matrix<T, N, M>
    where
        T: Mul<T, Output = T> + AddAssign<T> + Default + Clone,
    {
        type Output = Matrix<T, N, K>;

        fn mul(self, rhs: &Matrix<T, M, K>) -> Self::Output {
            self.mul_matrix(rhs)
        }
    }
    impl<const N: usize, const M: usize, const K: usize, T> Mul<Matrix<T, M, K>> for &Matrix<T, N, M>
    where
        T: Mul<T, Output = T> + AddAssign<T> + Default + Clone,
    {
        type Output = Matrix<T, N, K>;

        fn mul(self, rhs: Matrix<T, M, K>) -> Self::Output {
            self.mul_matrix(&rhs)
        }
    }
    impl<const N: usize, const M: usize, const K: usize, T> Mul<&Matrix<T, M, K>> for Matrix<T, N, M>
    where
        T: Mul<T, Output = T> + AddAssign<T> + Default + Clone,
    {
        type Output = Matrix<T, N, K>;

        fn mul(self, rhs: &Matrix<T, M, K>) -> Self::Output {
            self.mul_matrix(rhs)
        }
    }
    impl<const N: usize, const M: usize, const K: usize, T> Mul<Matrix<T, M, K>> for Matrix<T, N, M>
    where
        T: Mul<T, Output = T> + AddAssign<T> + Default + Clone,
    {
        type Output = Matrix<T, N, K>;

        fn mul(self, rhs: Matrix<T, M, K>) -> Self::Output {
            self.mul_matrix(&rhs)
        }
    }

    impl<const N: usize, const M: usize, T> Mul<T> for &Matrix<T, N, M>
    where
        T: Mul<Output = T> + Clone,
    {
        type Output = Matrix<T, N, M>;

        fn mul(self, rhs: T) -> Self::Output {
            self.map(|_, x| x * rhs.clone())
        }
    }
    impl<const N: usize, const M: usize, T> Mul<T> for Matrix<T, N, M>
    where
        T: Mul<Output = T> + Clone,
    {
        type Output = Matrix<T, N, M>;

        fn mul(self, rhs: T) -> Self::Output {
            &self * rhs
        }
    }
    // impl<const N: usize, const M: usize, T:Mul<T,Output = T>> Mul<Matrix<T, N, M>> for T {
    //     type Output = Matrix<T, N, M>;

    //     fn mul(self, rhs: Matrix<T, N, M>) -> Self::Output {
    //         rhs * self
    //     }
    // }
    // impl<'a, const N: usize, const M: usize, T: Mul<Matrix<T, N, M>, Output = Matrix<T, N, M>>>
    //     Mul<&Matrix<T, N, M>> for T
    // {
    //     type Output = Matrix<T, N, M>;

    //     fn mul(self, rhs: &Matrix<T, N, M>) -> Self::Output {
    //         rhs * self
    //     }
    // }
}
mod div {
    use std::ops::Div;

    use super::Matrix;

    impl<const N: usize, const M: usize, T> Div<T> for &Matrix<T, N, M>
    where
        T: Clone + Div<Output = T>,
    {
        type Output = Matrix<T, N, M>;

        fn div(self, rhs: T) -> Self::Output {
            self.map(|_, x| x / rhs.clone())
        }
    }
    impl<const N: usize, const M: usize, T> Div<T> for Matrix<T, N, M>
    where
        T: Clone + Div<Output = T>,
    {
        type Output = Matrix<T, N, M>;

        fn div(self, rhs: T) -> Self::Output {
            &self / rhs
        }
    }
}

pub type Matrix4<T> = Matrix<T, 4, 4>;
pub type Matrix3<T> = Matrix<T, 3, 3>;
pub type Vector4<T> = Matrix<T, 4, 1>;
pub type Vector3<T> = Matrix<T, 3, 1>;
