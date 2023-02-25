use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard, Mutex, Condvar, TryLockError},
};

use super::{
    components::{Component, ComponentCollection},
    entity::Entity,
};

#[derive(Debug)]
pub struct InsertError();

#[derive(Debug)]
pub struct RemoveError;

#[derive(Debug)]
pub enum ReadError {
    NotExist,
    CantRead,
}
#[derive(Debug)]
pub enum TryReadError {
    NotExist,
    CantRead,
    WaitForMutex,
}

pub type SafeCollection<T> = Arc<RwLock<ComponentCollection<T>>>;
fn create_safe_collection<T:Component>() -> SafeCollection<T> {
    Arc::new(RwLock::new(ComponentCollection::default()))
}

#[derive(Default)]
pub struct ComponentRegistry {
    data: HashMap<TypeId, Arc<dyn Any+Send+Sync>>,

    m:Arc<Mutex<()>>,
    cv:Arc<Condvar>


}



impl ComponentRegistry {
    pub fn get_vec<T: Component + 'static>(&self) -> Result<RwLockReadGuard<ComponentCollection<T>>,ReadError> {
        let b = match self.data.get(&TypeId::of::<T>()) {
            Some(it) => it,
            None => return Err(ReadError::NotExist),
        };
        let v = match b.downcast_ref::<SafeCollection<T>>() {
            Some(it) => it,
            None => return Err(ReadError::CantRead),
        };
        match v.read() {
            Ok(it) => Ok(it),
            Err(_) => Err(ReadError::CantRead),
        }
    }
    pub fn get_vec_mut<T: Component + 'static>(
        &self,
    ) -> Result<RwLockWriteGuard<ComponentCollection<T>>,ReadError> {
        let b = match self.data.get(&TypeId::of::<T>()) {
            Some(it) => it,
            None => return Err(ReadError::NotExist),
        };
        let v = match b.downcast_ref::<SafeCollection<T>>() {
            Some(it) => it,
            None => return Err(ReadError::CantRead),
        };
        match v.write() {
            Ok(it) => Ok(it),
            Err(_) => Err(ReadError::CantRead),
        }
    }

    pub fn try_get_vec<T: Component + 'static>(&self) -> Result<RwLockReadGuard<ComponentCollection<T>>,TryReadError> {
        let b = match self.data.get(&TypeId::of::<T>()) {
            Some(it) => it,
            None => return Err(TryReadError::NotExist),
        };
        let v = match b.downcast_ref::<SafeCollection<T>>() {
            Some(it) => it,
            None => return Err(TryReadError::CantRead),
        };
        match v.try_read() {
            Ok(it) => Ok(it),
            Err(TryLockError::WouldBlock) => Err(TryReadError::WaitForMutex),
            Err(_) => Err(TryReadError::CantRead),
        }
    }
    pub fn try_get_vec_mut<T: Component + 'static>(
        &self,
    ) -> Result<RwLockWriteGuard<ComponentCollection<T>>,TryReadError> {
        let b = match self.data.get(&TypeId::of::<T>()) {
            Some(it) => it,
            None => return Err(TryReadError::NotExist),
        };
        let v = match b.downcast_ref::<SafeCollection<T>>() {
            Some(it) => it,
            None => return Err(TryReadError::CantRead),
        };
        match v.try_write() {
            Ok(it) => Ok(it),
            Err(TryLockError::WouldBlock) => Err(TryReadError::WaitForMutex),
            Err(_) => Err(TryReadError::CantRead),
        }
    }


    pub fn insert<T: Component + 'static>(&mut self, e: Entity, t: T) -> Result<(),InsertError> {
        let b = self.data.get(&TypeId::of::<T>());
        let Some(b) = b else {
            self.data.insert(TypeId::of::<T>(), Arc::new(create_safe_collection::<T>()));    

            return  self.insert::<T>(e, t);
        };
        let Some(v) = b.downcast_ref::<SafeCollection<T>>() else {
            return Err(InsertError());
        };

        let Ok(mut g) = v.write() else {
            return Err(InsertError());
        };

        g.insert(e, t);

        Ok(())
    }

    pub fn get_conditional_mutex(&self) -> (Arc<Mutex<()>>,Arc<Condvar>) {
        (self.m.clone(),self.cv.clone())
    }
    
}


#[derive(Default)]
pub struct RegistryBuilder {
    pub components:ComponentRegistry
}
impl RegistryBuilder {
    pub fn build(self) -> Registry {
        Registry { components: Arc::new(RwLock::new(self.components)) }
    }
}

#[derive(Default)]
pub struct Registry {
    components: Arc<RwLock<ComponentRegistry>>,
}
impl Registry {
    pub fn get_component_registry(&self) -> Arc<RwLock<ComponentRegistry>> {
        self.components.clone()
    }

}

mod tests {
    use crate::matrix_engine::{components::Component, entity::Entity};

    use super::ComponentRegistry;


    struct A;
    impl Component for A {}
    #[test]
    fn test_reg() {
        let mut c = ComponentRegistry::default();
        let e = Entity::default();
        c.insert(e, A{}).unwrap();
        c.insert(e, A{}).unwrap();

        println!("{:?}",c.data);
    }
}


#[macro_export]
macro_rules! first {
    ($e:expr $(,es:expr)*) => {
        $e
    };
}

#[macro_export]
macro_rules! not_first {
    ($e:expr $(,es:expr)*) => {
        $(es,)*
    };
}


#[macro_export]
macro_rules! query {
    (read, $t:ty, $life:lifetime) => {
        RwLockReadGuard<$life,ComponentCollection<$t>>
    };
    (write, $t:ty,$life:lifetime) => {
        RwLockWriteGuard<$life,ComponentCollection<$t>>

    };
    (read, $t:ty) => {
        RwLockReadGuard<ComponentCollection<$t>>
    };
    (write, $type:ty) => {
        RwLockWriteGuard<ComponentCollection<$t>>

    };
    (read, $l:expr,$t:ty) => {
        {
            $l.try_get_vec::<$t>()?
        }
    };
    (write, $l:expr,$t:ty) => {
        {
            $l.try_get_vec_mut::<$t>()?
        }
    };

    ($args:expr,|$($pres:tt $names:tt:$types:ty),+| $func:block) => {
        {

        use std::sync::{MutexGuard,Mutex,RwLockReadGuard,RwLockWriteGuard,Condvar,Arc};
        use matrix_engine::{systems::SystemArgs,registry::{ComponentRegistry,SafeCollection,TryReadError},components::{ComponentCollection}};

            

            let Some(registry) = $args.read_components() else {
                panic!();
            };
            let (reg_mutex,reg_cv) = registry.get_conditional_mutex();

            {

                
                fn get<'a>(reg:&'a RwLockReadGuard<ComponentRegistry>) -> Result<($(query!($pres, $types,'a),)+),TryReadError> {
                    Ok(($(query!($pres,reg,$types)),+))
                }
                
                let mut guard = reg_mutex.lock().unwrap();
                let mut state = get(&registry);
                while let Err(ref e) = state {
                    match e{
                        TryReadError::CantRead |TryReadError::NotExist => break,
                        _ => {}
                    } 

                    guard = reg_cv.wait(guard).unwrap();
                    drop(state);
                    state = get(&registry);
                }
                drop(guard);   
                if let Ok(($($names),+)) = state {
                    
                    
                };
                // ($($names),+)
            };


            reg_cv.notify_all();
        
            

    }

    };
}
