#[derive(Debug)]
pub struct FastVec<ID: Eq, T> {
    data: Vec<Option<(ID, T)>>,
    free_indices: Vec<usize>,
}

impl<ID: Eq, T> Default for FastVec<ID, T> {
    fn default() -> Self {
        Self {
            data: Default::default(),
            free_indices: Default::default(),
        }
    }
}

impl<ID: Eq, T> FastVec<ID, T> {
    pub fn new() -> Self {
        FastVec {
            data: Vec::new(),
            free_indices: Vec::new(),
        }
    }

    pub fn push(&mut self, id: ID, value: T) -> (usize, &T) {
        if let Some(index) = self.free_indices.pop() {
            (index, &self.data[index].insert((id, value)).1)
        } else {
            self.data.push(Some((id, value)));
            (
                self.data.len() - 1,
                &self.data.last().as_ref().unwrap().as_ref().unwrap().1,
            )
        }
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index < self.data.len() {
            if self.data[index].is_some() {
                self.free_indices.push(index);
            }
            self.data[index].take().map(|(_, x)| x)
        } else {
            None
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)?.as_ref().map(|(_, x)| x)
    }

    pub fn get_index_by_id(&self, id: &ID) -> Option<(usize, &T)> {
        self.data.iter().enumerate().find_map(|(index, x)| {
            x.as_ref().and_then(|(_id, data)| {
                if _id.eq(id) {
                    Some((index, data))
                } else {
                    None
                }
            })
        })
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.data.get_mut(index)?.as_mut().map(|(_, x)| x)
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
            .filter_map(|(i, x)| x.as_ref().map(|(_, x)| x).map(move |x| (i, x)))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (usize, &mut T)> {
        self.data
            .iter_mut()
            .enumerate()
            .filter_map(|(i, x)| x.as_mut().map(|(_, x)| x).map(move |x| (i, x)))
    }
}
