use std::sync::{Arc, Mutex};

pub struct Storage<T> {
    data: Arc<Mutex<Option<Arc<T>>>>,
}


impl<T: Default> Default for Storage<T> {
    fn default() -> Self {
        Self { data: Arc::new(Mutex::new(Some(Arc::new(Default::default())))) }
    }
}
pub struct StorageReadGuard<T> {
    data: Arc<T>,
}
impl<T> StorageReadGuard<T> {
    pub fn get(&self) -> &T {
        &self.data
    }
    fn new(data: Arc<T>) -> Self {
        Self { data }
    }
}

pub struct StorageWriteGuard<T> {
    data: Option<T>,
    data_ref: Arc<Mutex<Option<Arc<T>>>>,
}
impl<T> StorageWriteGuard<T> {
    pub fn get(&self) -> &T {
                self.data.as_ref().expect("this should not be empty")

    }
    pub fn get_mut(&mut self) -> &mut T {
        self.data.as_mut().expect("this should not be empty")
    }
    fn new(data: T, data_ref: Arc<Mutex<Option<Arc<T>>>>) -> Self {
        Self { data:Some(data), data_ref }
    }
}
impl<T> Drop for StorageWriteGuard<T> {
    fn drop(&mut self) {
        let mut m = self.data_ref.lock().unwrap();
        let _ = m.insert(Arc::new(self.data.take().expect("this sould not be empty")));
    }
}

impl<T> Storage<T> {
    pub fn new(data:T) -> Self {
        Self{ 
            data:Arc::new(Mutex::new(Some(Arc::new(data))))
        }
    }
    pub fn read(&self) -> Option<StorageReadGuard<T>> {
        let data = self.data.lock().expect("this should not crash");
        data.clone().map(|x| StorageReadGuard::new(x))
    }
    pub fn write(&self) -> Option<StorageWriteGuard<T>> {
        let data_ref = self.data.clone();
        let mut data = data_ref.lock().expect("this should not crash");
        let Some(arc) = data.take() else {
            return None;
        };
        let Ok(t) = Arc::try_unwrap(arc) else {
            return None;       
        };
        drop(data);
        Some(StorageWriteGuard::new(t,data_ref))
    }
}

mod tests {

    #[test]
    fn test_storage() {
        use crate::components::storage::Storage;
        println!("adasda");
        
        let data1 = Storage::new(10);
        {
            assert_eq!(data1.read().unwrap().get(),&10);
            assert_eq!(data1.read().unwrap().get(),&10);
            assert_eq!(data1.read().unwrap().get(),&10);
            assert_eq!(data1.read().unwrap().get(),&10);
        }
        {
            let a1 = data1.write().unwrap();
            assert_eq!(a1.get(),&10);
            assert!(data1.write().is_none());
            drop(a1);
            assert!(data1.write().is_some());
        }
    }
}