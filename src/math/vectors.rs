use std::ops::{AddAssign, Div, Mul, Sub};

use num_traits::{Float, Zero};

use super::matrices::{Matrix, Vector3};

pub trait Vector<T> {
    fn normalized(&self) -> Self
    where
        T: Mul<Output = T> + AddAssign<T> + Float + Div<Output = T>;
    fn dot(&self, other: &Self) -> T
    where
        T: Zero + Mul<Output = T> + AddAssign<T> + Clone;
    fn distance_to_f32(&self, other: &Self) -> f32
    where
        T: Into<f32> + Clone;
    fn distance_to_zero_f32(&self) -> f32
    where
        T: Into<f32> + Clone;
    fn distance_to_f64(&self, other: &Self) -> f64
    where
        T: Into<f64> + Clone;
    fn distance_to_zero_f64(&self) -> f64
    where
        T: Into<f64> + Clone;
}

pub trait Vector1D<T> {
    fn x(&self) -> &T;
    fn x_mut(&mut self) -> &mut T;
}
pub trait Vector2D<T> {
    fn x(&self) -> &T;
    fn y(&self) -> &T;

    fn x_mut(&mut self) -> &mut T;
    fn y_mut(&mut self) -> &mut T;
}
pub trait Vector3D<T> {
    fn x(&self) -> &T;
    fn y(&self) -> &T;
    fn z(&self) -> &T;

    fn x_mut(&mut self) -> &mut T;
    fn y_mut(&mut self) -> &mut T;
    fn z_mut(&mut self) -> &mut T;

    fn cross(&self, other: &Self) -> Self
    where
        T: Mul<Output = T> + Sub<Output = T> + Clone;
}
pub trait Crossable {
    fn cross(&self, other: &Self) -> Self;
}
pub trait Vector4D<T> {
    fn x(&self) -> &T;
    fn y(&self) -> &T;
    fn z(&self) -> &T;
    fn w(&self) -> &T;

    fn x_mut(&mut self) -> &mut T;
    fn y_mut(&mut self) -> &mut T;
    fn z_mut(&mut self) -> &mut T;
    fn w_mut(&mut self) -> &mut T;
}

impl<T, const N: usize> Vector<T> for Matrix<T, N, 1> {
    fn normalized(&self) -> Self
    where
        T: Mul<Output = T> + AddAssign<T> + Float + Div<Output = T>,
    {
        let sum: T = Float::sqrt(self.dot(self));
        match sum.is_zero() {
            true => self.clone(),
            false => self / sum,
        }
    }

    fn dot(&self, other: &Self) -> T
    where
        T: Zero + Mul<Output = T> + AddAssign<T> + Clone,
    {
        let mut ans = T::zero();

        for i in 0..N {
            ans += self[i].clone() * other[i].clone();
        }

        ans
    }

    fn distance_to_f32(&self, other: &Self) -> f32
    where
        T: Into<f32> + Clone,
    {
        (self.clone_cast::<f32>() - other.clone_cast::<f32>())
            .into_iter()
            .map(|(_, x)| x * x)
            .sum::<f32>()
            .sqrt()
    }

    fn distance_to_zero_f32(&self) -> f32
    where
        T: Into<f32> + Clone,
    {
        self.clone_cast::<f32>()
            .into_iter()
            .map(|(_, x)| x*x)
            .sum::<f32>()
            .sqrt()
    }

    fn distance_to_f64(&self, other: &Self) -> f64
    where
        T: Into<f64> + Clone,
    {
        (self.clone_cast::<f64>() - other.clone_cast::<f64>())
            .into_iter()
            .map(|(_, x)| x * x)
            .sum::<f64>()
            .sqrt()
    }

    fn distance_to_zero_f64(&self) -> f64
    where
        T: Into<f64> + Clone,
    {
        self.clone_cast::<f64>()
            .into_iter()
            .map(|(_, x)| x * x)
            .sum::<f64>()
            .sqrt()
    }
}

impl<T> Vector1D<T> for Matrix<T, 1, 1> {
    fn x(&self) -> &T {
        &self[0]
    }

    fn x_mut(&mut self) -> &mut T {
        &mut self[0]
    }
}

impl<T> Vector2D<T> for Matrix<T, 2, 1> {
    fn x(&self) -> &T {
        &self[0]
    }

    fn y(&self) -> &T {
        &self[1]
    }

    fn x_mut(&mut self) -> &mut T {
        &mut self[0]
    }

    fn y_mut(&mut self) -> &mut T {
        &mut self[1]
    }
}
impl<T> Vector3D<T> for Matrix<T, 3, 1> {
    fn x(&self) -> &T {
        &self[0]
    }

    fn y(&self) -> &T {
        &self[1]
    }

    fn x_mut(&mut self) -> &mut T {
        &mut self[0]
    }

    fn y_mut(&mut self) -> &mut T {
        &mut self[1]
    }

    fn z(&self) -> &T {
        &self[2]
    }

    fn z_mut(&mut self) -> &mut T {
        &mut self[2]
    }
    fn cross(&self, other: &Self) -> Self
    where
        T: Mul<Output = T> + Sub<Output = T> + Clone,
    {
        [[
            self.y().clone() * other.z().clone() - self.z().clone() * other.y().clone(),
            self.z().clone() * other.x().clone() - self.x().clone() * other.z().clone(),
            self.x().clone() * other.y().clone() - self.y().clone() * other.x().clone(),
        ]]
        .into()
    }
}

impl<T> Vector4D<T> for Matrix<T, 4, 1> {
    fn x(&self) -> &T {
        &self[0]
    }

    fn y(&self) -> &T {
        &self[1]
    }

    fn x_mut(&mut self) -> &mut T {
        &mut self[0]
    }

    fn y_mut(&mut self) -> &mut T {
        &mut self[1]
    }

    fn z(&self) -> &T {
        &self[2]
    }

    fn z_mut(&mut self) -> &mut T {
        &mut self[2]
    }

    fn w(&self) -> &T {
        &self[3]
    }

    fn w_mut(&mut self) -> &mut T {
        &mut self[3]
    }
}

impl<T: From<f32>> Vector3<T> {
    pub fn up() -> Self {
        Vector3::<T>::from([[T::from(0.0), T::from(1.0), T::from(0.0)]])
    }
}

trait Quaternion<T>: Vector4D<T> {}

impl<T> Quaternion<T> for Matrix<T, 4, 1> {}
