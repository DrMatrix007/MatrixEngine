use std::{cell::UnsafeCell, ops::{Deref, DerefMut}, sync::Mutex};

pub struct RwState<T> {
    data: Box<UnsafeCell<T>>,
    state: State,
}

#[derive(Debug)]
pub enum RwStateAccessError {
    NotAvailable,
}

#[derive(Debug)]
pub enum RwStateConsumeError {
    WrongValue,
    WrongState,
}

impl<T> RwState<T> {
    pub fn new(data: T) -> Self {
        Self {
            data: Box::new(UnsafeCell::new(data)),
            state: State::default(),
        }
    }

    pub fn read(&mut self) -> Result<RwReadState<T>, RwStateAccessError> {
        let state = &mut self.state;
        match &mut *state {
            State::Read(i) => {
                *i += 1;
                Ok(RwReadState::new(self.data.get()))
            }
            state @ State::Ready => {
                *state = State::Read(1);
                Ok(RwReadState::new(self.data.get()))
            }
            State::Write => Err(RwStateAccessError::NotAvailable),
        }
    }
    pub fn write(&mut self) -> Result<RwWriteState<T>, RwStateAccessError> {
        let state = &mut self.state;
        match &mut *state {
            state @ State::Ready => {
                *state = State::Write;
                Ok(RwWriteState::new(self.data.get()))
            }
            State::Read(_) | State::Write => Err(RwStateAccessError::NotAvailable),
        }
    }

    pub fn consume_read(&mut self, read: RwReadState<T>) -> Result<(), RwStateConsumeError> {
        if read.ptr != self.data.get() {
            return Err(RwStateConsumeError::WrongValue);
        }

        let state = &mut self.state;

        match &mut *state {
            State::Read(i) if *i >= 2 => *i -= 1,
            State::Read(i) if *i == 1 => *state = State::Ready,
            _ => return Err(RwStateConsumeError::WrongState),
        };
        Ok(())
    }
    pub fn consume_write(&mut self, read: RwWriteState<T>) -> Result<(), RwStateConsumeError> {
        if read.ptr != self.data.get() {
            return Err(RwStateConsumeError::WrongValue);
        }

        let state = &mut self.state;

        match &mut *state {
            state @ State::Write => *state = State::Ready,
            _ => return Err(RwStateConsumeError::WrongState),
        };
        Ok(())
    }
}

impl<T> From<T> for RwState<T> {
    fn from(value: T) -> Self {
        RwState::new(value)
    }
}

#[derive(Debug)]
enum State {
    Ready,
    Read(isize),
    Write,
}
impl Default for State {
    fn default() -> Self {
        Self::Ready
    }
}

#[derive(Debug)]
#[must_use]
pub struct RwReadState<T> {
    ptr: *const T,
}

impl<T> RwReadState<T> {
    pub fn new(ptr: *const T) -> Self {
        Self { ptr }
    }
}

impl<T> Deref for RwReadState<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}

#[derive(Debug)]
#[must_use]
pub struct RwWriteState<T> {
    ptr: *mut T,
}

impl<T> RwWriteState<T> {
    pub fn new(ptr: *mut T) -> Self {
        Self { ptr }
    }
}
impl<T> Deref for RwWriteState<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}

impl<T> DerefMut for RwWriteState<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.ptr }
    }
}

mod tests {

    #[test]
    fn test_rw_state() {
        use super::RwState;

        let mut data = RwState::new(10);

        let a1 = data.read().unwrap();
        let a2 = data.read().unwrap();

        data.write().unwrap_err();

        data.consume_read(a1).unwrap();
        data.consume_read(a2).unwrap();
        
        let a3 = data.write().unwrap();

        data.read().unwrap_err();

        data.consume_write(a3).unwrap();
        
        let a4 = data.read().unwrap();
        data.consume_read(a4).unwrap();
    }
}
