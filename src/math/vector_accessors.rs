use super::{
    matrix::{Matrix, Vector2, Vector3, Vector4},
    number::Number,
};

impl<T: Number> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        let storage = [[x], [y]];
        Matrix::from_storage(storage)
    }

    pub fn x(&self) -> &T {
        &self[(0, 0)]
    }

    pub fn y(&self) -> &T {
        &self[(1, 0)]
    }

    pub fn x_mut(&mut self) -> &mut T {
        &mut self[(0, 0)]
    }

    pub fn y_mut(&mut self) -> &mut T {
        &mut self[(1, 0)]
    }
}

impl<T: Number> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        let storage = [[x], [y], [z]];
        Matrix::from_storage(storage)
    }

    pub fn x(&self) -> &T {
        &self[(0, 0)]
    }

    pub fn y(&self) -> &T {
        &self[(1, 0)]
    }

    pub fn z(&self) -> &T {
        &self[(2, 0)]
    }

    pub fn x_mut(&mut self) -> &mut T {
        &mut self[(0, 0)]
    }

    pub fn y_mut(&mut self) -> &mut T {
        &mut self[(1, 0)]
    }

    pub fn z_mut(&mut self) -> &mut T {
        &mut self[(2, 0)]
    }
}

impl<T: Number> Vector4<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        let storage = [[x], [y], [z], [w]];
        Matrix::from_storage(storage)
    }

    pub fn x(&self) -> &T {
        &self[(0, 0)]
    }

    pub fn y(&self) -> &T {
        &self[(1, 0)]
    }

    pub fn z(&self) -> &T {
        &self[(2, 0)]
    }

    pub fn w(&self) -> &T {
        &self[(3, 0)]
    }

    pub fn x_mut(&mut self) -> &mut T {
        &mut self[(0, 0)]
    }

    pub fn y_mut(&mut self) -> &mut T {
        &mut self[(1, 0)]
    }

    pub fn z_mut(&mut self) -> &mut T {
        &mut self[(2, 0)]
    }

    pub fn w_mut(&mut self) -> &mut T {
        &mut self[(3, 0)]
    }
}
