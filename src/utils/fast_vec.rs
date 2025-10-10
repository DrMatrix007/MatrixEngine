#[derive(Debug)]
pub struct FastVec<T> {
    data: Vec<Option<T>>,
    free_indices: Vec<usize>,
}

impl<T> Default for FastVec<T> {
    fn default() -> Self {
        Self {
            data: Default::default(),
            free_indices: Default::default(),
        }
    }
}

impl<T> FastVec<T> {
    pub fn new() -> Self {
        FastVec {
            data: Vec::new(),
            free_indices: Vec::new(),
        }
    }

    pub fn push(&mut self, value: T) {
        if let Some(index) = self.free_indices.pop() {
            self.data[index] = Some(value);
        } else {
            self.data.push(Some(value));
        }
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index < self.data.len() {
            if self.data[index].is_some() {
                self.free_indices.push(index);
            }
            self.data[index].take()
        } else {
            None
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)?.as_ref()
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.data.get_mut(index)?.as_mut()
    }

    pub fn len(&self) -> usize {
        self.data.len() - self.free_indices.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, &T)> {
        self.data
            .iter()
            .enumerate()
            .filter_map(|(i, x)| x.as_ref().map(move |x| (i, x)))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (usize, &mut T)> {
        self.data
            .iter_mut()
            .enumerate()
            .filter_map(|(i, x)| x.as_mut().map(move |x| (i, x)))
    }
}
