
pub trait Number: num_traits::Num+Clone {}

impl<T:num_traits::Num+Clone> Number for T{}