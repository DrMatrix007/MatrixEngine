pub mod entity_builder;

use std::sync::atomic::AtomicUsize;

static COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct Entity(usize);

impl Entity {
    pub fn new() -> Self {
        Self(COUNTER.fetch_add(1, std::sync::atomic::Ordering::Acquire))
    }
}

impl Default for Entity {
    fn default() -> Self {
        Self::new()
    }
}
