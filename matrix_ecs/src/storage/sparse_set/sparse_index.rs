pub trait SparseIndex:
    Into<usize> + From<usize> + Copy + Clone + PartialEq + Eq + Ord + PartialOrd
{
}

impl<T: Into<usize> + From<usize> + Copy + Clone + PartialEq + Eq + Ord + PartialOrd> SparseIndex
    for T
{
}
