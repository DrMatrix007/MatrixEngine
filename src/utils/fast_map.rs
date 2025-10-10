use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FastMapID(usize);

#[derive(Debug)]
pub struct FastMap<T> {
    data: Vec<Option<T>>,
    empty_queue: VecDeque<FastMapID>,
}

impl<T: Default> Default for FastMap<T> {
    fn default() -> Self {
        Self {
            data: Default::default(),
            empty_queue: Default::default(),
        }
    }
}

impl<T> FastMap<T> {
    pub fn push(&mut self, data: T) -> FastMapID {
        if let Some(index) = self.empty_queue.pop_front() {
            *self.data.get_mut(index.0).unwrap() = Some(data);
            index
        } else {
            self.data.push(Some(data));
            FastMapID(self.data.len() - 1)
        }
    }
    pub fn remove(&mut self, index: FastMapID) -> Option<T> {
        let curr_len = self.data.len();

        if curr_len == 0 {
            return None;
        }

        match index {
            FastMapID(index) if index == curr_len - 1 => self.data.pop().and_then(|x| x),
            index => {
                self.empty_queue.push_back(index);
                self.data.get_mut(index.0).and_then(|x| x.take())
            }
        }
    }

    pub fn get(&self, index: FastMapID) -> Option<&T> {
        self.data.get(index.0).and_then(|x| x.as_ref())
    }

    pub fn get_mut(&mut self, index: FastMapID) -> Option<&mut T> {
        self.data.get_mut(index.0).and_then(|x| x.as_mut())
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn access_at(&self, index: usize) -> Option<&T> {
        self.data.get(index).and_then(|x| x.as_ref())
    }

    pub fn access_at_mut(&mut self, index: usize) -> Option<&mut T> {
        self.data.get_mut(index).and_then(|x| x.as_mut())
    }
}
