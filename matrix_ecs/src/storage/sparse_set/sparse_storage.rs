pub trait SparseStorage<T>: AsRef<[T]> + AsMut<[T]> {}
impl<T, Storage: AsRef<[T]> + AsMut<[T]>> SparseStorage<T> for Storage {}
