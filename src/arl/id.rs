use std::hash::Hash;

pub trait ID: Hash + Eq + Clone + Copy {}
impl<T: Hash + Eq + Clone + Copy> ID for T {}
