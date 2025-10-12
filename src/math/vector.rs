use num_traits::Float;

use crate::math::matrix::{ColVector, RowVector};

pub trait MatrixVectorType {}

pub struct MatrixVectorRowType;
impl MatrixVectorType for MatrixVectorRowType {}
pub struct MatrixVectorColType;
impl MatrixVectorType for MatrixVectorColType {}

pub trait Vector<Type: MatrixVectorType> {
    type Scalar;

    fn dot(&self, rhs: &Self) -> Self::Scalar;
    fn normalized(&self) -> Option<Self>
    where
        Self: Sized;
}

impl<const N: usize, T: Float> Vector<MatrixVectorColType> for ColVector<N, T> {
    type Scalar = T;

    fn dot(&self, rhs: &Self) -> Self::Scalar {
        (0..N)
            .map(|i| self[i] * rhs[i])
            .fold(T::zero(), |a, b| a + b)
    }

    fn normalized(&self) -> Option<Self> {
        let norm = (0..N)
            .map(|i| self[i].powi(2))
            .fold(T::zero(), |a, b| a + b);
        if T::epsilon() >= norm {
            None
        } else {
            Some(*self / norm)
        }
    }
}

impl<const N: usize, T: Float> Vector<MatrixVectorRowType> for RowVector<N, T> {
    type Scalar = T;

    fn dot(&self, rhs: &Self) -> Self::Scalar {
        (0..N)
            .map(|i| self[i] * rhs[i])
            .fold(T::zero(), |a, b| a + b)
    }

    fn normalized(&self) -> Option<Self> {
        let norm = (0..N)
            .map(|i| self[i].powi(2))
            .fold(T::zero(), |a, b| a + b);
        if T::epsilon() >= norm {
            None
        } else {
            Some(*self / norm)
        }
    }
}

pub trait CrossableVector {
    fn cross(&self, rhs: &Self) -> Self;
}

impl<T: Float> CrossableVector for RowVector<3, T> {
    fn cross(&self, rhs: &Self) -> Self {
        RowVector::<3, T>::new([[
            self[1] * rhs[2] - self[2] * rhs[1],
            self[2] * rhs[0] - self[0] * rhs[2],
            self[0] * rhs[1] - self[1] * rhs[0],
        ]])
    }
}

impl<T: Float> CrossableVector for ColVector<3, T> {
    fn cross(&self, rhs: &Self) -> Self {
        ColVector::<3, T>::new([
            [self[1] * rhs[2] - self[2] * rhs[1]],
            [self[2] * rhs[0] - self[0] * rhs[2]],
            [self[0] * rhs[1] - self[1] * rhs[0]],
        ])
    }
}
