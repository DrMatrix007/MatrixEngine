use std::hash::Hash;

pub trait IDable: Hash + Eq + Clone + Copy {}
impl<T: Hash + Eq + Clone + Copy> IDable for T {}
