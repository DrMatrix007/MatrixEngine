pub mod sparse_set;
pub mod table;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum StorageType {
    Table,
    SparseSet,
}