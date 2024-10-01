use super::number::Number;

pub trait MatrixStoragable<T: Number, const M: usize, const N: usize> {
    fn zeros() -> Self;
    fn ones() -> Self;
    fn build_with(f: impl FnMut() -> T) -> Self;
    fn build_with_pos(f: impl FnMut(usize, usize) -> T) -> Self;

    fn get(&self, pos: (usize, usize)) -> &T;
    fn get_mut(&mut self, pos: (usize, usize)) -> &mut T;
}

impl<T: Number, const M: usize, const N: usize> MatrixStoragable<T, M, N> for [[T; N]; M] {
    fn zeros() -> Self {
        Self::build_with(|| T::zero())
    }

    fn ones() -> Self {
        Self::build_with(|| T::one())
    }

    fn build_with(mut f: impl FnMut() -> T) -> Self {
        [[(); N]; M].map(|x| x.map(|_| f()))
    }

    fn build_with_pos(mut f: impl FnMut(usize, usize) -> T) -> Self {
        let mut m = 0;
        let mut n = 0;
        [[(); N]; M].map(|x| {
            n = 0;
            let ans = x.map(|_| {
                let ans = f(m, n);
                n += 1;
                ans
            });
            m += 1;
            ans
        })
    }

    fn get(&self, (m, n): (usize, usize)) -> &T {
        &self[m][n]
    }

    fn get_mut(&mut self, (m, n): (usize, usize)) -> &mut T {
        &mut self[m][n]
    }
}
