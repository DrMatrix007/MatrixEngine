use num_traits::Float;

use super::matrices::{Matrix4, Vector4};

pub struct Quaternion<T> {
    mat: Matrix4<T>,
}

impl<T: Float> Quaternion<T> {
    pub fn new(vec: Vector4<T>) -> Self {
        let (a, b, c, d) = (vec[0], vec[1], vec[2], vec[3]);
        Self {
            mat: Matrix4::from([[a, -b, -c, -d], [b, a, -d, c], [c, d, a, -b], [d, -c, b, a]]),
        }
    }
    pub fn new_with(a: T, b: T, c: T, d: T) -> Self {
        let mut ans = Self {
            mat: Matrix4::zeros(),
        };
        ans.set_a(a);
        ans.set_b(b);
        ans.set_c(c);
        ans.set_d(d);

        ans
    }

    pub fn mat(&self) -> &Matrix4<T> {
        &self.mat
    }

    pub fn set_a(&mut self, a: T) {
        self.mat[(0, 0)] = a;
        self.mat[(1, 1)] = a;
        self.mat[(2, 2)] = a;
        self.mat[(3, 3)] = a;
    }
    pub fn set_b(&mut self, b: T) {
        self.mat[(0, 1)] = -b;
        self.mat[(1, 0)] = b;
        self.mat[(2, 3)] = -b;
        self.mat[(3, 2)] = b;
    }
    pub fn set_c(&mut self, c: T) {
        self.mat[(0, 2)] = -c;
        self.mat[(1, 3)] = c;
        self.mat[(2, 0)] = c;
        self.mat[(3, 1)] = -c;
    }
    pub fn set_d(&mut self, d: T) {
        self.mat[(0, 3)] = -d;
        self.mat[(1, 2)] = -d;
        self.mat[(2, 1)] = d;
        self.mat[(3, 0)] = d;
    }
}
