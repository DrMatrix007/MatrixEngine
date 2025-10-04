use std::cell::UnsafeCell;

pub enum LockableState {
    Available,
    Read(usize),
    Write,
}

impl LockableState {
    pub fn read(&mut self) -> Result<(), ()> {
        match self {
            LockableState::Available => *self = Self::Read(1),
            LockableState::Read(reads) => *reads += 1,
            LockableState::Write => return Err(()),
        };
        Ok(())
    }

    pub fn consume_read(&mut self) -> Result<(), ()> {
        match self {
            LockableState::Read(1) => *self = Self::Available,
            LockableState::Read(data) => *data -= 1,
            _ => return Err(()),
        };
        Ok(())
    }

    pub fn write(&mut self) -> Result<(), ()> {
        match self {
            LockableState::Available => *self = Self::Write,
            _ => return Err(()),
        };
        Ok(())
    }

    pub fn consume_write(&mut self) -> Result<(), ()> {
        match self {
            LockableState::Write => *self = Self::Available,
            _ => return Err(()),
        };
        Ok(())
    }
}

pub struct LockableWriteGuard<T> {
    data: *mut T,
}

impl<T> LockableWriteGuard<T> {
    pub(crate) fn new(data: *mut T) -> Self {
        Self { data }
    }
}

impl<T> AsMut<T> for LockableWriteGuard<T> {
    fn as_mut(&mut self) -> &mut T {
        unsafe { &mut *self.data }
    }
}

impl<T> AsRef<T> for LockableWriteGuard<T> {
    fn as_ref(&self) -> &T {
        unsafe { &*self.data }
    }
}

pub struct LockableReadGuard<T> {
    data: *const T,
}

impl<T> LockableReadGuard<T> {
    pub(crate) fn new(data: *const T) -> Self {
        Self { data }
    }
}

impl<T> AsRef<T> for LockableReadGuard<T> {
    fn as_ref(&self) -> &T {
        unsafe { &*self.data }
    }
}

pub struct Lockable<T> {
    state: LockableState,
    data: Box<UnsafeCell<T>>,
}

impl<T> Lockable<T> {
    pub fn new(data: T) -> Self {
        Self {
            state: LockableState::Available,
            data: Box::new(UnsafeCell::new(data)),
        }
    }
    pub fn read(&mut self) -> Option<LockableReadGuard<T>> {
        self.state
            .read()
            .ok()
            .map(|_| LockableReadGuard::new(self.data.get()))
    }
    pub fn write(&mut self) -> Option<LockableWriteGuard<T>> {
        self.state
            .write()
            .ok()
            .map(|_| LockableWriteGuard::new(self.data.get()))
    }

    pub fn consume_read(&mut self, _: LockableReadGuard<T>) -> Result<(), ()> {
        self.state.consume_read()
    }

    pub fn consume_write(&mut self, _: LockableWriteGuard<T>) -> Result<(), ()> {
        self.state.consume_write()
    }
}
