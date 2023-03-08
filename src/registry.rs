use std::{
    any::{Any, TypeId},
    collections::HashMap,
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
    WaitForMutex,
}


#[derive(Default)]
pub struct ComponentRegistry {
    data: HashMap<TypeId, Box<dyn Any>>,
}



impl ComponentRegistry {
    pub fn get_vec<T: Component + 'static>(&self) -> Result<&ComponentCollection<T>,ReadError> {
        let b = match self.data.get(&TypeId::of::<T>()) {
            Some(it) => it,
            None => return Err(ReadError::NotExist),
        };
        match b.downcast_ref::<ComponentCollection<T>>() {
            Some(it) => Ok(it),
            None => Err(ReadError::CantRead),
        }
    }
    pub fn get_vec_mut<T: Component + 'static>(
        &mut self,
    ) -> Result<&mut ComponentCollection<T>,ReadError> {
        let b = match self.data.get_mut(&TypeId::of::<T>()) {
            Some(it) => it,
            None => return Err(ReadError::NotExist),
        };
        match b.downcast_mut::<ComponentCollection<T>>() {
            Some(it) => Ok(it),
            None => Err(ReadError::CantRead),
        }
        
    }



    pub fn insert<T: Component + 'static>(&mut self, e: Entity, t: T) -> Result<(),InsertError> {
        let b = self.data.get_mut(&TypeId::of::<T>());
        let Some(b) = b else {
            self.data.insert(TypeId::of::<T>(), Box::<ComponentCollection<T>>::default());    

            return self.insert::<T>(e, t);
        };
        let Some(v) = b.downcast_mut::<ComponentCollection<T>>() else {
            return Err(InsertError());
        };
        v.insert(e, t);

        Ok(())
    }
    
}

#[derive(Default)]
pub struct Registry {
    components: ComponentRegistry,
}
impl Registry {
    pub fn get_component_registry(&self) -> &ComponentRegistry {
        &self.components
    }
    
    pub fn get_component_registry_mut(&mut self) -> &mut ComponentRegistry {
        &mut self.components
    }

}

#[allow(unused_imports)]
mod tests {

    use crate::{components::Component, registry::ComponentRegistry, entity::Entity};



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
        &$life ComponentCollection<$t>
    };
    (write, $t:ty,$life:lifetime) => {
        &$life mut ComponentCollection<$t>

    };
    (read, $t:ty) => {
        RwLockReadGuard<ComponentCollection<$t>>
    };
    (write, $type:ty) => {
        RwLockWriteGuard<ComponentCollection<$t>>

    };
    (read, $l:expr,$t:ty) => {
        {
            $l.get_vec::<$t>()?
        }
    };
    (write, $l:expr,$t:ty) => {
        {
            $l.get_vec_mut::<$t>()?
        }
    };
    (read $e:expr) => {
        $e.iter()
    };
    (write $e:expr) => {
        $e.iter_mut()
    };
    (read $e:expr, $ent:expr) => {
        $e.get($ent)
    };
    (write $e:expr, $ent:expr) => {
        $e.get_mut($ent)
    };

    ($args:expr,|$pre:tt $name:tt:$type:ty $(,$pres:tt $names:tt:$types:ty),*| $func:block) => {
        {

        use std::sync::{MutexGuard,Mutex,RwLockReadGuard,RwLockWriteGuard,Condvar,Arc};
        use matrix_engine::{systems::SystemArgs,registry::{ComponentRegistry,ReadError},components::{ComponentCollection}};

        

            let registry = $args.components();
            #[allow(unused_mut,unused_variables)]
            {
            fn get_components<'a>(reg:&'a ComponentRegistry) -> Result<(query!($pre, $type,'a),$(query!($pres, $types,'a),)*),ReadError> {
                Ok((query!($pre,reg,$type),$(query!($pres,reg,$types)),*))
            }
            let mut state = get_components(&registry);
                match state    
                {
                    Ok((mut $name,$(mut $names),*))=> {
                        for (entity,$name) in query!($pre $name) {
                            let ($($names,)*) = ($(match query!($pres $names,e){
                                Some(a)=>a,
                                None=>continue,
                            }),*);
                            $func
                        }
                    },
                    Err(r) => {
                        match r {
                            ReadError::CantRead => panic!("cant read data!"),
                            _=>{}
                        } 
                    },
                }    
            };


        
            

    }

    };
}
