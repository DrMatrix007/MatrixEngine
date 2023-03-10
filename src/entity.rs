use std::sync::atomic::AtomicUsize;

static COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Copy, PartialEq, Eq, Hash,Debug)]
pub struct Entity(usize);

impl Default for Entity {
    fn default() -> Self {
        Self(COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
    }
}
