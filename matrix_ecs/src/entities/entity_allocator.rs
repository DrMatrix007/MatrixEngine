use crate::entities::Entity;
use std::collections::VecDeque;

#[derive(Default)]
pub struct EntityAllocator {
    current: u64,
    remains: VecDeque<u64>,
}

impl EntityAllocator {
    pub fn allocate(&mut self) -> Entity {
        Entity(match self.remains.pop_back() {
            Some(value) => value,
            None => {
                let res = self.current;
                self.current += 1;
                res
            }
        })
    }

    pub fn deallocate(&mut self, entity: Entity) {
        if entity.0 == self.current - 1 {
            self.current -= 1;
        } else {
            self.remains.push_back(entity.0);
        }
    }
}
