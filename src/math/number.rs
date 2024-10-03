use std::ops::Neg;

pub trait Number: num_traits::Num + Clone + Neg<Output = Self> {}

impl<T: num_traits::Num + Clone + Neg<Output = T>> Number for T {}
