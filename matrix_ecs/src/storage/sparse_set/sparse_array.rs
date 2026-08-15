use crate::storage::sparse_set::sparse_index::SparseIndex;
use std::marker::PhantomData;
use crate::storage::sparse_set::sparse_storage::SparseStorage;

pub struct SparseArrayWithStorage<I, T, Storage: SparseStorage<Option<T>>> {
    inner: Storage,
    _marker: PhantomData<(I, T)>,
}

pub type SparseArray<I, T> = SparseArrayWithStorage<I, T, Vec<Option<T>>>;
pub type ImmutableSparseArray<I, T> = SparseArrayWithStorage<I, T, Box<[Option<T>]>>;

impl<I, T, Storage: SparseStorage<Option<T>> + Default> Default
    for SparseArrayWithStorage<I, T, Storage>
{
    fn default() -> Self {
        Self {
            inner: Storage::default(),
            _marker: PhantomData,
        }
    }
}

impl<I, T, Storage: SparseStorage<Option<T>>> SparseArrayWithStorage<I, T, Storage> {
    pub fn new(inner: Storage) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }
    pub fn get(&self, index: I) -> Option<&T>
    where
        I: SparseIndex,
    {
        self.inner
            .as_ref()
            .get(index.into())
            .and_then(|x| x.as_ref())
    }

    pub fn get_mut(&mut self, index: I) -> Option<&mut T>
    where
        I: SparseIndex,
    {
        self.inner
            .as_mut()
            .get_mut(index.into())
            .and_then(|x| x.as_mut())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (I, &mut T)>
    where
        I: SparseIndex,
    {
        self.inner
            .as_mut()
            .iter_mut()
            .enumerate()
            .filter_map(|(index, value)| value.as_mut().map(|value| (I::from(index), value)))
    }
}

impl<I, V> SparseArrayWithStorage<I, V, Vec<Option<V>>> {
    pub fn insert(&mut self, index: I, value: V)
    where
        I: SparseIndex,
    {
        let index = index.into();
        if index >= self.inner.len() {
            self.inner.resize_with(index + 1, || None);
        }

        self.inner[index] = Some(value);
    }

    pub fn remove(&mut self, index: I) -> Option<V>
    where
        I: SparseIndex,
    {
        let res = self.inner.get_mut(index.into()).and_then(Option::take);

        self.try_reduce_size();

        res
    }

    fn try_reduce_size(&mut self) {
        self.inner.truncate(
            self.inner
                .iter()
                .rposition(|x| x.is_some())
                .map_or(0, |x| x + 1),
        )
    }

    pub fn into_immutable(self) -> ImmutableSparseArray<I, V> {
        ImmutableSparseArray::new(self.inner.into_boxed_slice())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_and_remove_sparse_values() {
        let mut array = SparseArray::<usize, i32>::default();

        array.insert(2, 20);
        array.insert(5, 50);

        assert_eq!(array.get(2), Some(&20));
        assert_eq!(array.get(3), None);
        assert_eq!(array.get(5), Some(&50));

        assert_eq!(array.remove(5), Some(50));
        assert_eq!(array.get(5), None);
    }

    #[test]
    fn iter_mut_returns_indices_and_values() {
        let mut array = SparseArray::<usize, i32>::default();

        array.insert(1, 10);
        array.insert(4, 40);

        for (_, value) in array.iter_mut() {
            *value += 1;
        }

        assert_eq!(array.get(1), Some(&11));
        assert_eq!(array.get(4), Some(&41));
    }
}