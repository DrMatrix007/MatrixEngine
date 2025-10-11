use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

#[derive(Debug)]
pub enum LockableState<T> {
    Available(Arc<T>),
    Read(Arc<T>),
    Write,
}

#[derive(Debug, Clone, Copy)]
pub enum LockableError {
    NotAvailable,
    CantConsume,
}
impl<T> LockableState<T> {
    pub fn read(&mut self) -> Result<Arc<T>, LockableError> {
        match std::mem::replace(self, LockableState::Write) {
            LockableState::Available(data) => {
                *self = LockableState::Read(data.clone());
                Ok(data)
            }
            LockableState::Read(arc) => {
                *self = LockableState::Read(arc.clone());
                Ok(arc)
            }
            LockableState::Write => {
                *self = LockableState::Write;
                Err(LockableError::NotAvailable)
            }
        }
    }

    pub fn consume_read(&mut self, arc: Arc<T>) -> Result<(), LockableError> {
        if Arc::strong_count(&arc) == 2 {
            match std::mem::replace(    self, LockableState::Write) {
                LockableState::Read(current) => {
                    if Arc::ptr_eq(&arc, &current) {
                        *self = LockableState::Available(current);
                        return Ok(());
                    }
                }
                data => {
                    *self = data;
                }
            }
        }
        Ok(())
    }

    pub fn write(&mut self) -> Result<Arc<T>, LockableError> {
        match std::mem::replace(self, LockableState::Write) {
            LockableState::Available(data) => Ok(data),
            LockableState::Read(arc) => {
                *self = LockableState::Read(arc);
                Err(LockableError::NotAvailable)
            }
            LockableState::Write => {
                *self = LockableState::Write;
                Err(LockableError::NotAvailable)
            }
        }
    }

    pub fn consume_write(&mut self, data: Arc<T>) -> Result<(), LockableError> {
        if let LockableState::Write = self {
            *self = LockableState::Available(data);
            Ok(())
        } else {
            Err(LockableError::CantConsume)
        }
    }
}

pub struct LockableWriteGuard<T> {
    data: Arc<T>,
}
unsafe impl<T: Send> Send for LockableWriteGuard<T> {}

impl<T> LockableWriteGuard<T> {
    pub(crate) fn new(data: Arc<T>) -> Self {
        Self { data }
    }
}
impl<T> AsRef<T> for LockableWriteGuard<T> {
    fn as_ref(&self) -> &T {
        self.data.as_ref()
    }
}

impl<T> AsMut<T> for LockableWriteGuard<T> {
    fn as_mut(&mut self) -> &mut T {
        // TODO: change back to the unsafe 
        unsafe { Arc::get_mut_unchecked(&mut self.data) }
        // Arc::get_mut(&mut self.data).unwrap()
    }
}

impl<T> DerefMut for LockableWriteGuard<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.as_mut()
    }
}

impl<T> Deref for LockableWriteGuard<T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.as_ref()
    }
}

pub struct LockableReadGuard<T> {
    data: Arc<T>,
}

unsafe impl<T: Send> Send for LockableReadGuard<T> {}

impl<T> LockableReadGuard<T> {
    pub(crate) fn new(data: Arc<T>) -> Self {
        Self { data }
    }
}

impl<T> Deref for LockableReadGuard<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> AsRef<T> for LockableReadGuard<T> {
    fn as_ref(&self) -> &T {
        self.data.as_ref()
    }
}

#[derive(Debug)]
pub struct Lockable<T> {
    state: LockableState<T>,
}

impl<T: Default> Default for Lockable<T> {
    fn default() -> Self {
        Self {
            state: LockableState::Available(Arc::new(Default::default())),
        }
    }
}

impl<T> Lockable<T> {
    pub fn new(data: T) -> Self {
        Self {
            state: LockableState::Available(Arc::new(data)),
        }
    }
    pub fn read(&mut self) -> Result<LockableReadGuard<T>, LockableError> {
        self.state.read().map(|arc| LockableReadGuard::new(arc))
    }
    pub fn write(&mut self) -> Result<LockableWriteGuard<T>, LockableError> {
        self.state.write().map(|arc| LockableWriteGuard::new(arc))
    }

    pub fn consume_read(&mut self, guard: LockableReadGuard<T>) -> Result<(), LockableError> {
        self.state.consume_read(guard.data)
    }

    pub fn consume_write(&mut self, guard: LockableWriteGuard<T>) -> Result<(), LockableError> {
        self.state.consume_write(guard.data)
    }

    pub fn can_read(&self) -> bool {
        matches!(
            self.state,
            LockableState::Available(_) | LockableState::Read(_)
        )
    }

    pub fn can_write(&self) -> bool {
        matches!(self.state, LockableState::Available(_))
    }
}
