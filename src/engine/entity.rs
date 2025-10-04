use std::{sync::atomic::AtomicUsize, usize};

type EntityId = usize;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Entity {
    id: EntityId,
}

static COUNTER: AtomicUsize = AtomicUsize::new(0);

impl Entity {
    pub fn new() -> Self {
        Self::from_id(COUNTER.fetch_add(1, std::sync::atomic::Ordering::AcqRel))
    }
    pub fn from_id(id: EntityId) -> Self {
        Self { id }
    }

    pub fn id(&self) -> EntityId {
        self.id
    }
}
