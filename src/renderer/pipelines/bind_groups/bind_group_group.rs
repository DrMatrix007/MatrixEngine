use crate::impl_all;

use super::bind_group::MatrixBindGroupable;
pub trait MatrixBindGroupableGroupable {}

macro_rules! impl_group_group {
    ($($t:tt)*) => {
        impl<$($t:MatrixBindGroupable,)*> MatrixBindGroupableGroupable for ($($t,)*) {

        }
    }
}

impl_all!(impl_group_group);
