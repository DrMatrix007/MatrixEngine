use std::{
    borrow::{Borrow, BorrowMut},
    sync::{Arc, Mutex},
};

pub struct RwStorage<T> {
    data: Arc<Mutex<Option<Arc<T>>>>,
}

impl<T> RwStorage<T> {
    pub fn new(data: T) -> Self {
        Self {
            data: Arc::new(Mutex::new(Option::Some(Arc::new(data)))),
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

pub struct ReadStorageGuard<T> {
    data: Arc<T>,
    mtx: Arc<Mutex<Option<Arc<T>>>>,
}

impl<T> Drop for ReadStorageGuard<T> {
    fn drop(&mut self) {
        *self.mtx.lock().expect("the mutex should not be poisoned") = Some(self.data.clone());
    }
}

impl<T> ReadStorageGuard<T> {
    fn new(data: Arc<T>, mtx: Arc<Mutex<Option<Arc<T>>>>) -> Self {
        Self { data, mtx }
    }
}
impl<T> AsRef<T> for ReadStorageGuard<T> {
    fn as_ref(&self) -> &T {
        &self.data
    }
}
impl<T> std::ops::Deref for ReadStorageGuard<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}
impl<T> Borrow<T> for ReadStorageGuard<T> {
    fn borrow(&self) -> &T {
        self
    }
}

/////////////
///

pub struct WriteStorageGuard<T> {
    data: Option<T>,
    mtx: Arc<Mutex<Option<Arc<T>>>>,
}

impl<T> Drop for WriteStorageGuard<T> {
    fn drop(&mut self) {
        *self.mtx.lock().expect("the mutex should be poisoned") = Some(Arc::new(
            self.data.take().expect("the value should not be empty"),
        ));
    }
}

impl<T> WriteStorageGuard<T> {
    fn new(data: T, mtx: Arc<Mutex<Option<Arc<T>>>>) -> Self {
        Self {
            data: Some(data),
            mtx,
        }
    }
}
impl<T> AsRef<T> for WriteStorageGuard<T> {
    fn as_ref(&self) -> &T {
        &self.data.as_ref().expect("the value should not be empty")
    }
}
impl<T> std::ops::Deref for WriteStorageGuard<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}
impl<T> AsMut<T> for WriteStorageGuard<T> {
    fn as_mut(&mut self) -> &mut T {
        self.data.as_mut().expect("the value should not be empty")
    }
}
impl<T> std::ops::DerefMut for WriteStorageGuard<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}
impl<T> BorrowMut<T> for WriteStorageGuard<T> {
    fn borrow_mut(&mut self) -> &mut T {
        self
    }
}
impl<T> Borrow<T> for WriteStorageGuard<T> {
    fn borrow(&self) -> &T {
        self
    }
}

#[test]
fn test() {
    let a = RwStorage::new(10);

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
