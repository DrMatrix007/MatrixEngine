use std::sync::atomic::AtomicU64;

static COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct Entity(u64);

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
