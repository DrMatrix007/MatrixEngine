use std::ops::{Deref, DerefMut};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum DataStateAccessError {
    NotAvailableError,
    WrongFuckingData,
}
#[derive(Debug)]
enum State {
    Ready,
    Reading { readers: i32 },
    Writing,
}

#[derive(Debug)]
pub struct DataState<T> {
    data: Box<T>,
    state: State,
}

impl<T: Default> Default for DataState<T> {
    fn default() -> Self {
        Self {
            data: Box::default(),
            state: State::Ready,
        }
    }
}
impl<T> DataState<T> {
    pub fn new(t: T) -> Self {
        Self {
            data: Box::new(t),
            state: State::Ready,
        }
    }

    pub fn get(&self) -> Result<&T, DataStateAccessError> {
        match self.state {
            State::Writing => Err(DataStateAccessError::NotAvailableError),
            _ => Ok(&*self.data),
        }
    }
    pub fn get_mut(&mut self) -> Result<&mut T, DataStateAccessError> {
        match self.state {
            State::Ready => Ok(&mut *self.data),
            _ => Err(DataStateAccessError::NotAvailableError),
        }
    }

    pub fn read(&mut self) -> Result<ReadDataState<T>, DataStateAccessError> {
        match &mut self.state {
            State::Ready => {
                self.state = State::Reading { readers: 1 };
                Ok(ReadDataState::new(&*self.data))
            }
            State::Reading { readers } => {
                *readers += 1;
                Ok(ReadDataState::new(&*self.data))
            }
            State::Writing => Err(DataStateAccessError::NotAvailableError),
        }
    }
    pub fn write(&mut self) -> Result<WriteDataState<T>, DataStateAccessError> {
        match self.state {
            State::Reading { .. } | State::Writing => Err(DataStateAccessError::NotAvailableError),
            State::Ready => {
                self.state = State::Writing;
                Ok(WriteDataState::new(&mut *self.data))
            }
        }
    }

    pub fn consume_write(&mut self, data: WriteDataState<T>) -> Result<(), DataStateAccessError> {
        if &*self.data as *const _ != data.data {
            return Err(DataStateAccessError::WrongFuckingData);
        }
        match self.state {
            State::Writing => {
                self.state = State::Ready;
                Ok(())
            }
            State::Reading { .. } | State::Ready => Err(DataStateAccessError::NotAvailableError),
        }
    }
    pub fn consume_read(&mut self, data: ReadDataState<T>) -> Result<(), DataStateAccessError> {
        if &*self.data as *const _ != data.data {
            return Err(DataStateAccessError::WrongFuckingData);
        }
        match &mut self.state {
            State::Reading { readers } => {
                *readers -= 1;
                if *readers <= 0 {
                    self.state = State::Ready;
                }
                Ok(())
            }
            State::Writing | State::Ready => Err(DataStateAccessError::NotAvailableError),
        }
    }
    pub fn can_read(&self) -> bool {
        match self.state {
            State::Ready | State::Reading { .. } => true,
            State::Writing => false,
        }
    }
    pub fn can_write(&self) -> bool {
        match self.state {
            State::Ready => true,
            State::Reading { .. } | State::Writing => false,
        }
    }
}

#[derive(Debug)]
#[must_use = "needs to be consumed"]
pub struct ReadDataState<T> {
    data: *const T,
}
unsafe impl<T: Send> Send for ReadDataState<T> {}

impl<T> ReadDataState<T> {
    fn new(data: *const T) -> Self {
        Self { data }
    }
}

impl<T> Deref for ReadDataState<T> {
    fn deref(&self) -> &T {
        unsafe { &*self.data }
    }

    type Target = T;
}

#[must_use = "needs to be consumed"]
#[derive(Debug)]
pub struct WriteDataState<T> {
    data: *mut T,
}

impl<T> WriteDataState<T> {
    fn new(data: *mut T) -> Self {
        Self { data }
    }
}
unsafe impl<T: Send> Send for WriteDataState<T> {}

impl<C> Deref for WriteDataState<C> {
    type Target = C;
    fn deref(&self) -> &C {
        unsafe { &*self.data }
    }
}
impl<C> DerefMut for WriteDataState<C> {
    fn deref_mut(&mut self) -> &mut C {
        unsafe { &mut *self.data }
    }
}
