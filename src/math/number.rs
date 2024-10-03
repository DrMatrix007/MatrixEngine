use std::ops::Neg;

pub trait Number: num_traits::Num + Clone + Neg<Output = Self> + 'static {}

impl<T: num_traits::Num + Clone + Neg<Output = T> + 'static> Number for T {}
