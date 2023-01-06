use std::{
    any::{Any, TypeId},
    collections::HashMap,
    fmt::Debug,
    marker::PhantomData,
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use super::{
    components::{Component, ComponentVec, Entity},
    systems::System,
};

pub struct InsertError<T>(PhantomData<T>);
pub struct RemoveError<T>(PhantomData<T>);
impl<T> InsertError<T> {
    pub(super) fn new() -> Self {
        Self(PhantomData {})
    }
}
impl<T> Debug for InsertError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("InsertError").field(&self.0).finish()
    }
}
impl<T> RemoveError<T> {
    pub(super) fn new() -> Self {
        Self(PhantomData {})
    }
}
impl<T> Debug for RemoveError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("RemoveError").field(&self.0).finish()
    }
}

#[derive(Default)]
pub struct ComponentRegistry {
    data: HashMap<TypeId, Box<dyn Any>>,
}
unsafe impl Send for ComponentRegistry {}
unsafe impl Sync for ComponentRegistry {}

impl ComponentRegistry {
    pub fn get<T: Component + 'static>(&self) -> Option<RwLockReadGuard<ComponentVec<T>>> {
        let v = self.data.get(&TypeId::of::<T>())?;
        return v.downcast_ref::<SafeVec<T>>()?.read().ok();
    }
    pub fn get_mut<T: Component + 'static>(&self) -> Option<RwLockWriteGuard<ComponentVec<T>>> {
        let v = self.data.get(&TypeId::of::<T>())?;
        return v.downcast_ref::<SafeVec<T>>()?.write().ok();
    }

    pub fn insert<T: Component + 'static>(
        &mut self,
        e: Entity,
        t: T,
    ) -> Result<(), InsertError<T>> {
        let Some(b) = self.data.get_mut(&TypeId::of::<T>()) else {
            self.data.insert(TypeId::of::<T>(), Box::new(Arc::new(RwLock::new(ComponentVec::<T>::new()))));
            return self.insert(e, t);
        };
        let Some(v) = b.downcast_mut::<SafeVec<T>>() else {
            return Err(InsertError::new());
        };
        let Ok(mut v) = v.write() else {
            return  Err(InsertError::new());
        };
        v.insert(e, t);
        Ok(())
    }
}

#[macro_export]
//#[warn(non_snake_case)]
macro_rules!    query {

    ($reg:expr, read $type:ty) =>{
        $reg.get::<$type>()
    };
    ($reg:expr, write $type:ty) =>{
        $reg.get_mut::<$type>()
    };
    (read $vec:expr) => {
        $vec.iter()
    };
    (write $vec:expr) => {
        $vec.iter_mut()
    };
    (read $type:ty,$vec:expr, $entity:expr) =>{
         match $vec.get($entity) {
            Some(a) => a,
            None => continue,
        }
    };

    (write $type:ty,$vec:expr,$entity:expr) =>{
        match $vec.get_mut($entity) {
            Some(a) => a,
            None => continue,
        }
    };
    ($reg:expr,|$pre:tt $name:tt:$type:ty| $func:block, $sorter:expr) => {
        {
            #[allow(unused_variables)]
            if let Some(mut $name) = query!($reg,$pre $type) {
                let mut vec = Vec::new();
                for (entity,$name) in (query!($pre $name)){
                    vec.push((entity,$name));
                }
                vec.sort_by($sorter);
                for (entity,$name) in vec.into_iter() {
                    $func
                }
            }
        }
    };
    ($reg:expr,|$pre:tt $name:tt:$type:ty| $func:block) => {
        query!($reg,|$pre $name:$type| $func,|_,_|std::cmp::Ordering::Equal)
    };

    ($reg:expr,|$pre:tt $name:tt:$type:ty,$($pres:tt $names:tt:$types:ty),+| $func:block,$sorter:expr) => {
        {

            #[allow(non_snake_case)]
            let q = ||{
                let ($(mut $names,)*) = ($(match query!($reg,$pres $types){
                    Some(a) => a,
                    None => {
                       return;
                    }
                },)*);

                if let Some(mut $name) = query!($reg,$pre $type) {
                    let mut vec = Vec::new();
                    for (entity,i) in query!($pre $name) {
                        let ($name,$($names),*) = (i,$(query!($pres $types,$names,entity )),*);
                        vec.push(($name,$($names),*));
                    }
                    vec.sort_by($sorter);
                    for (($name,$($names),*)) in vec.into_iter() {
                        $func
                    }
                }
            };
            q();
        }
    };
    ($reg:expr,|$pre:tt $name:tt:$type:ty,$($pres:tt $names:tt:$types:ty),+| $func:block) => {
        query!($reg,|$pre $name:$type,$($pres $names:$types),+| $func,|_,_|std::cmp::Ordering::Equal)
    };
}

#[derive(Default)]
pub struct Registry {
    pub(super) data: Arc<RwLock<ComponentRegistry>>,
    pub(super) systems: HashMap<TypeId, Box<dyn System>>,
}
type SafeVec<T> = Arc<RwLock<ComponentVec<T>>>;
impl Registry {
    pub fn new() -> Self {
        Self {
            data: Default::default(),
            systems: Default::default(),
        }
    }

    pub fn insert_system<T: System + 'static>(&mut self, t: T) {
        self.systems.insert(TypeId::of::<T>(), Box::new(t));
    }
    pub fn insert<T: Component + 'static>(&self, e: Entity, t: T) -> Result<(), InsertError<T>> {
        let Ok(mut g) = self.data.write() else {
            return Err(InsertError::new());
        };
        g.insert(e, t)
    }
    pub fn read<T: Component + 'static, Ans>(
        &self,
        f: impl FnOnce(RwLockReadGuard<ComponentVec<T>>) -> Ans,
    ) -> Option<Ans> {
        let Some(v) = self.data.read().ok() else {
            return None;
        };
        let Some(v) = v.get::<T>() else {
            return None;
        };
        Some(f(v))
    }
    pub fn write<T: Component + 'static, Ans>(
        &self,
        f: impl FnOnce(RwLockWriteGuard<ComponentVec<T>>) -> Ans,
    ) -> Option<Ans> {
        let Some(v) = self.data.read().ok() else {
            return None;
        };
        let Some(v) = v.get_mut::<T>() else {
            return None;
        };
        Some(f(v))
    }
}
