use std::{
    borrow::{Borrow, BorrowMut},
    sync::{Arc, Mutex},
};

pub struct Storage<T: ?Sized> {
    data: Arc<Mutex<Option<Arc<Box<T>>>>>,
}

impl<T> Storage<T> {
    pub fn new(data: T) -> Self {
        Self::from_boxed(Box::new(data))
    }
}

impl<T: ?Sized> Storage<T> {
    pub fn from_boxed(b: Box<T>) -> Self {
        Self {
            data: Arc::new(Mutex::new(Option::Some(Arc::new(b)))),
        }
    }
    pub fn try_read(&self) -> Option<ReadStorageGuard<T>> {
        self.data
            .lock()
            .expect("the mutex should not be poisoned")
            .as_ref()
            .map(|x| ReadStorageGuard::new(Arc::clone(&x), Arc::clone(&self.data)))
    }

    pub fn try_write(&self) -> Option<WriteStorageGuard<T>> {
        let mut data = self.data.lock().expect("the mutex should not be poisoned");
        let out = data.take()?;

        match Arc::try_unwrap(out) {
            Ok(t) => Some(WriteStorageGuard::new(t, self.data.clone())),
            Err(t) => {
                *data = Some(t);
                None
            }
        }
    }
}

pub struct ReadStorageGuard<T: ?Sized> {
    data: Arc<Box<T>>,
    mtx: Arc<Mutex<Option<Arc<Box<T>>>>>,
}

impl<T: ?Sized> Drop for ReadStorageGuard<T> {
    fn drop(&mut self) {
        *self.mtx.lock().expect("the mutex should not be poisoned") = Some(self.data.clone());
    }
}

impl<T: ?Sized> ReadStorageGuard<T> {
    fn new(data: Arc<Box<T>>, mtx: Arc<Mutex<Option<Arc<Box<T>>>>>) -> Self {
        Self { data, mtx }
    }
}
impl<T: ?Sized> AsRef<T> for ReadStorageGuard<T> {
    fn as_ref(&self) -> &T {
        &self.data
    }
}
impl<T: ?Sized> std::ops::Deref for ReadStorageGuard<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}
impl<T: ?Sized> Borrow<T> for ReadStorageGuard<T> {
    fn borrow(&self) -> &T {
        self
    }
}

pub struct WriteStorageGuard<T: ?Sized> {
    data: Option<Box<T>>,
    mtx: Arc<Mutex<Option<Arc<Box<T>>>>>,
}

impl<T: ?Sized> Drop for WriteStorageGuard<T> {
    fn drop(&mut self) {
        *self.mtx.lock().expect("the mutex should be poisoned") = Some(Arc::new(
            self.data.take().expect("the value should not be empty"),
        ));
    }
}

impl<T: ?Sized> WriteStorageGuard<T> {
    fn new(data: Box<T>, mtx: Arc<Mutex<Option<Arc<Box<T>>>>>) -> Self {
        Self {
            data: Some(data),
            mtx,
        }
    }
}
impl<T: ?Sized> AsRef<T> for WriteStorageGuard<T> {
    fn as_ref(&self) -> &T {
        &self.data.as_ref().expect("the value should not be empty")
    }
}
impl<T: ?Sized> std::ops::Deref for WriteStorageGuard<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}
impl<T: ?Sized> AsMut<T> for WriteStorageGuard<T> {
    fn as_mut(&mut self) -> &mut T {
        self.data.as_mut().expect("the value should not be empty")
    }
}
impl<T: ?Sized> std::ops::DerefMut for WriteStorageGuard<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}
impl<T: ?Sized> BorrowMut<T> for WriteStorageGuard<T> {
    fn borrow_mut(&mut self) -> &mut T {
        self
    }
}
impl<T: ?Sized> Borrow<T> for WriteStorageGuard<T> {
    fn borrow(&self) -> &T {
        self
    }
}

#[test]
fn test() {
    let a = Storage::new(10);

    let b = a.try_write().unwrap();
    assert!(a.try_read().is_none());
    assert!(a.try_read().is_none());
    assert!(a.try_read().is_none());
    drop(b);
    let c = a.try_read().unwrap();
    assert!(a.try_read().is_some());
    assert!(a.try_read().is_some());
    assert!(a.try_read().is_some());
    assert!(a.try_read().is_some());
    drop(c);
    let b = a.try_write().unwrap();
    assert!(a.try_read().is_none());
    assert!(a.try_read().is_none());
    assert!(a.try_read().is_none());
    drop(b);
    let _c = a.try_read().unwrap();
    assert!(a.try_read().is_some());
    assert!(a.try_read().is_some());
    assert!(a.try_read().is_some());
    assert!(a.try_read().is_some());
}
