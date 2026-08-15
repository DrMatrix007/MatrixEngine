use crate::storage::sparse_set::sparse_array::{SparseArrayWithStorage};
use crate::storage::sparse_set::sparse_index::SparseIndex;
use crate::storage::sparse_set::sparse_storage::SparseStorage;
use std::marker::PhantomData;

mod sparse_array;
mod sparse_index;
mod sparse_storage;

pub struct SparseSetWithStorage<
    I,
    V,
    IndicesStorage: SparseStorage<I>,
    DenseStorage: SparseStorage<V>,
    SparsesStorage: SparseStorage<Option<usize>>,
> {
    indices: IndicesStorage,
    dense: DenseStorage,
    sparse: SparseArrayWithStorage<I, usize, SparsesStorage>,
    _marker: PhantomData<V>,
}

pub type SparseSet<I, V> = SparseSetWithStorage<I, V, Vec<I>, Vec<V>, Vec<Option<usize>>>;
pub type ImmutableSparseSet<I, V> =
    SparseSetWithStorage<I, V, Box<[I]>, Box<[V]>, Box<[Option<usize>]>>;

impl<
    I,
    V,
    DensesStorage: SparseStorage<V> + Default,
    SparsesStorage: SparseStorage<I> + Default,
    SparseReverseStorage: SparseStorage<Option<usize>> + Default,
> Default for SparseSetWithStorage<I, V, SparsesStorage, DensesStorage, SparseReverseStorage>
{
    fn default() -> Self {
        Self {
            dense: DensesStorage::default(),
            indices: SparsesStorage::default(),
            sparse: Default::default(),
            _marker: PhantomData,
        }
    }
}

impl<
    I,
    V,
    DensesStorage: SparseStorage<V>,
    SparsesStorage: SparseStorage<I>,
    SparseReverseStorage: SparseStorage<Option<usize>>,
> SparseSetWithStorage<I, V, SparsesStorage, DensesStorage, SparseReverseStorage>
{
    pub fn len(&self) -> usize {
        self.dense.as_ref().len()
    }

    pub fn contains(&self, index: I) -> bool
    where
        I: PartialEq,
    {
        self.indices.as_ref().contains(&index)
    }

    pub fn get(&self, index: I) -> Option<&V>
    where
        I: SparseIndex,
    {
        self.sparse.get(index).map(|index| {
            #[cfg(debug_assertions)]
            return self.dense.as_ref().get(*index).unwrap();

            #[cfg(not(debug_assertions))]
            return unsafe { self.dense.as_ref().get_unchecked(*index) };
        })
    }

    pub fn get_mut(&mut self, index: I) -> Option<&mut V>
    where
        I: SparseIndex,
    {
        self.indices.as_ref().get(index.into()).map(|index| {
            #[cfg(debug_assertions)]
            return self.dense.as_mut().get_mut(index.clone().into()).unwrap();

            #[cfg(not(debug_assertions))]
            return unsafe { self.dense.as_mut().get_unchecked_mut(index.clone().into()) };
        })
    }

    pub fn indices(&self) -> &[I] {
        self.indices.as_ref()
    }

    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.dense.as_ref().into_iter()
    }

    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut V> {
        self.dense.as_mut().into_iter()
    }

    pub fn iter(&self) -> impl Iterator<Item = (I, &V)>
    where
        I: Clone,
    {
        self.indices
            .as_ref()
            .iter()
            .cloned()
            .zip(self.dense.as_ref().iter())
    }
}

impl<I, V> SparseSetWithStorage<I, V, Vec<I>, Vec<V>, Vec<Option<usize>>> {
    pub fn insert(&mut self, index: I, value: V)
    where
        I: SparseIndex,
    {
        if let Some(dense_index) = self.sparse.get(index) {
            #[cfg(debug_assertions)]
            {
                *self
                    .dense
                    .get_mut(*dense_index)
                    .expect("dense_index must be within bounds!") = value;
            }

            #[cfg(not(debug_assertions))]
            {
                *unsafe { self.dense.get_unchecked_mut(*dense_index) } = value;
            }
        } else {
            self.sparse.insert(index.clone(), self.dense.len());
            self.indices.push(index);
            self.dense.push(value);
        }
    }

    pub fn remove(&mut self, index: I) -> Option<V>
    where
        I: SparseIndex,
    {
        self.sparse.remove(index).map(|dense_index| {
            let is_last = dense_index == self.dense.len() - 1;

            self.indices.swap_remove(dense_index);
            let res = self.dense.swap_remove(dense_index);
            if !is_last {
                let swapped_index = self.indices[dense_index];
                *self.sparse.get_mut(swapped_index).unwrap() = dense_index;
            }

            res
        })
    }

    pub fn into_immutable(self) -> ImmutableSparseSet<I, V> {
        ImmutableSparseSet {
            sparse: self.sparse.into_immutable(),
            indices: self.indices.into_boxed_slice(),
            dense: self.dense.into_boxed_slice(),
            _marker: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_update_and_get() {
        let mut set = SparseSet::<usize, &str>::default();

        set.insert(10, "ten");
        set.insert(20, "twenty");

        assert_eq!(set.len(), 2);
        assert!(set.contains(10));
        assert_eq!(set.get(10), Some(&"ten"));
        assert_eq!(set.get(20), Some(&"twenty"));

        set.insert(10, "updated");

        assert_eq!(set.len(), 2);
        assert_eq!(set.get(10), Some(&"updated"));
    }

    #[test]
    fn remove_keeps_remaining_values_accessible() {
        let mut set = SparseSet::<usize, &str>::default();

        set.insert(10, "ten");
        set.insert(20, "twenty");
        set.insert(30, "thirty");

        assert_eq!(set.remove(20), Some("twenty"));

        assert_eq!(set.len(), 2);
        assert_eq!(set.get(10), Some(&"ten"));
        assert_eq!(set.get(30), Some(&"thirty"));
        assert!(!set.contains(20));
    }

    #[test]
    fn iter_returns_indices_and_values() {
        let mut set = SparseSet::<usize, i32>::default();

        set.insert(2, 20);
        set.insert(5, 50);

        let items: Vec<_> = set.iter().map(|(i, v)| (i, *v)).collect();

        assert_eq!(items, vec![(2, 20), (5, 50)]);
        assert_eq!(set.indices(), &[2, 5]);
        assert_eq!(set.values().copied().collect::<Vec<_>>(), vec![20, 50]);
    }
}