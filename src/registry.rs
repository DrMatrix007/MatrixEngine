<<<<<<< HEAD
use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use crate::resources::ResourceManager;

use super::{
    components::{Component, ComponentCollection},
    entity::Entity,
};

#[derive(Debug)]
pub struct InsertError();
=======
use crate::{
    components::{IComponentCollection, Component, ComponentCollection},
    queries::query::{Query, QueryData, Action}, entity::Entity,
};
use std::{any::TypeId, collections::HashMap};

#[derive(Debug)]
pub struct InsertError;
>>>>>>> temp

#[derive(Debug)]
pub struct RemoveError;

#[derive(Debug)]
pub enum ReadError {
    NotExist,
    CantRead,
<<<<<<< HEAD
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
    
=======
}

pub enum ComponentCollectionState {
    Available(Box<dyn IComponentCollection>),
    ReadOnly(Box<dyn IComponentCollection>, i32),
    Taken,
}
pub enum QueryError {
    CantRead,
    Taken,
    Empty,
}

#[derive(Default)]
pub struct ComponentRegistry {
    data: HashMap<TypeId, ComponentCollectionState>,
}

impl ComponentRegistry {
    // pub fn get_vec<T: Component + 'static>(&self) -> Result<&ComponentCollection<T>,ReadError> {
    //     let b = match self.data.get(&TypeId::of::<T>()) {
    //         Some(it) => it,
    //         None => return Err(ReadError::NotExist),
    //     };

    //     match b.downcast_ref::<ComponentCollection<T>>() {
    //         Some(it) => Ok(it),
    //         None => Err(ReadError::CantRead),
    //     }
    // }
    // pub fn get_vec_mut<T: Component + 'static>(
    //     &mut self,
    // ) -> Result<&mut ComponentCollection<T>,ReadError> {
    //     let b = match self.data.get_mut(&TypeId::of::<T>()) {
    //         Some(it) => it,
    //         None => return Err(ReadError::NotExist),
    //     };
    //     match b.downcast_mut::<ComponentCollection<T>>() {
    //         Some(it) => Ok(it),
    //         None => Err(ReadError::CantRead),
    //     }
    // }

    // pub fn insert<T: Component + 'static>(&mut self, e: Entity, t: T) -> Result<(),InsertError> {
    //     let Some(b) = self.data.get_mut(&TypeId::of::<T>()) else {
    //         self.data.insert(TypeId::of::<T>(), Box::<ComponentCollection::<T>>::default());

    //         return  self.insert::<T>(e, t);
    //     };
    //     let Some(v) = b.downcast_mut::<ComponentCollection<T>>() else {
    //         return Err(InsertError);
    //     };

    //     v.insert(e, t);

    //     Ok(())
    // }

    pub fn query(&mut self, q: &Query) -> Result<QueryData, QueryError> {
        let mut ans = HashMap::default();
        for action in q.data.iter() {
            match action {
                crate::queries::query::Action::Read(id) => {
                    let Some(vec) = self.data.get(id) else {
                        return Err(QueryError::Empty);
                    };
                    if let ComponentCollectionState::Taken = vec {
                        return Err(QueryError::Taken);
                    }
                }
                crate::queries::query::Action::Write(id) => {
                    let Some(vec) = self.data.get(id) else {
                        return Err(QueryError::Empty);
                    };
                    match vec {
                        ComponentCollectionState::Taken
                        | ComponentCollectionState::ReadOnly(_, _) => {
                            return Err(QueryError::Taken)
                        }
                        _ => {}
                    }
                }
            }
        }
        for action in q.data.iter() {
            match action {
                crate::queries::query::Action::<TypeId>::Read(id) => {
                    let data = self.data.remove(id).unwrap();
                    ans.insert(
                        *id,
                        match data {
                            ComponentCollectionState::Available(a) => {
                                self.data.insert(
                                    *id,
                                    ComponentCollectionState::ReadOnly(a.clone_vec(), 1),
                                );
                                Action::Read(a)
                            }
                            ComponentCollectionState::ReadOnly(a, count) => {
                                self.data.insert(
                                    *id,
                                    ComponentCollectionState::ReadOnly(a.clone_vec(), count + 1),
                                );
                                Action::Read(a)
                            }
                            _ => {
                                panic!("should not be here!");
                            }
                        },
                    );
                }
                crate::queries::query::Action::<TypeId>::Write(id) => {
                    let data = self.data.remove(id).unwrap();
                    ans.insert(
                        *id,
                        match data {
                            ComponentCollectionState::Available(a) => {
                                self.data.insert(*id, ComponentCollectionState::Taken);
                                Action::Write(a.clone_vec())
                            }
                            _ => {
                                panic!("should not be here!");
                            }
                        },
                    );
                }
            }
        }

        Ok(QueryData::with(ans))
    }
    pub fn update_query_result(&mut self, r: QueryData) {
        for (k, v) in r.data.into_iter() {
            match self.data.get_mut(&k) {
                Some(data) => match data {
                    ComponentCollectionState::Available(_) => {
                        panic!("this value should not be available if we get it from a query data.")
                    }
                    ComponentCollectionState::ReadOnly(_, i) => *i = (*i - 1).max(0),
                    ComponentCollectionState::Taken => {
                        self.data.insert(k, ComponentCollectionState::Available(v.unpack()));
                    }
                },
                None => {
                    self.data.insert(k, ComponentCollectionState::Available(v.unpack()));
                }
            }
        }
    }
>>>>>>> temp
}

#[derive(Default)]
pub struct Registry {
<<<<<<< HEAD
    components: ComponentRegistry,
    resources: ResourceManager
=======
    pub components: ComponentRegistry,
>>>>>>> temp
}
impl Registry {
    pub fn get_component_registry(&self) -> &ComponentRegistry {
        &self.components
    }
    
<<<<<<< HEAD
    pub fn get_component_registry_mut(&mut self) -> &mut ComponentRegistry {
        &mut self.components
    }
    pub fn get_resource_manager(&self) -> &ResourceManager {
        &self.resources
    }
    
    pub fn get_resource_manager_mut(&mut self) -> &mut ResourceManager {
        &mut self.resources
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

    ($registry:expr,  $pre:tt $type:ident $(,$pres:tt $types:ident)*) => {
        {

        use std::sync::{MutexGuard,Mutex,RwLockReadGuard,RwLockWriteGuard,Condvar,Arc};
        use matrix_engine::{systems::SystemArgs,registry::{ComponentRegistry,ReadError},components::{ComponentCollection}};

            let mut ans = Vec::new();

            #[allow(unused_mut,unused_variables)]
            {
            fn get_components<'a>(reg:&'a ComponentRegistry) -> Result<(query!($pre, $type,'a),$(query!($pres, $types,'a),)*),ReadError> {
                Ok((query!($pre,reg,$type),$(query!($pres,reg,$types)),*))
            }
            
            ans
       };
    }
}
=======
}

#[derive(Default)]
pub struct RegistryBuilder {
    components: HashMap<TypeId, Box<dyn IComponentCollection>>,
}

impl RegistryBuilder {
    pub fn insert<T: Component + 'static>(&mut self, e: Entity, t: T) -> Result<(),InsertError> {
        let b = self.components.get_mut(&TypeId::of::<T>());
        let Some(b) = b else {
            self.components.insert(TypeId::of::<T>(), Box::<ComponentCollection<T>>::default());    

            return self.insert::<T>(e, t);
        };
        let Some(v) = b.as_any_mut().downcast_mut::<ComponentCollection<T>>() else {
            return Err(InsertError);
        };
        v.insert(e, t);

        Ok(())
    }

    pub fn build(self) -> Registry {
        Registry { components: ComponentRegistry { data: self.components.into_iter()
            .map(|(x,y)|(x,ComponentCollectionState::Available(y))).collect() } }
    }
}
// mod tests {
//     use crate::{components::Component, entity::Entity};

//     use super::ComponentRegistry;

//     struct A;
//     impl Component for A {}
//     #[test]
//     fn test_reg() {
//         let mut c = ComponentRegistry::default();
//         let e = Entity::default();
//         c.insert(e, A{}).unwrap();
//         c.insert(e, A{}).unwrap();

//         println!("{:?}",c.data);
//     }
// }

// #[macro_export]
// macro_rules! first {
//     ($e:expr $(,es:expr)*) => {
//         $e
//     };
// }

// #[macro_export]
// macro_rules! not_first {
//     ($e:expr $(,es:expr)*) => {
//         $(es,)*
//     };
// }

// #[macro_export]
// macro_rules! query {
//     (read, $t:ty, $life:lifetime) => {
//         RwLockReadGuard<$life,ComponentCollection<$t>>
//     };
//     (write, $t:ty,$life:lifetime) => {
//         RwLockWriteGuard<$life,ComponentCollection<$t>>

//     };
//     (read, $t:ty) => {
//         RwLockReadGuard<ComponentCollection<$t>>
//     };
//     (write, $type:ty) => {
//         RwLockWriteGuard<ComponentCollection<$t>>

//     };
//     (read, $l:expr,$t:ty) => {
//         {
//             $l.try_get_vec::<$t>()?
//         }
//     };
//     (write, $l:expr,$t:ty) => {
//         {
//             $l.try_get_vec_mut::<$t>()?
//         }
//     };
//     (read $e:expr) => {
//         $e.iter()
//     };
//     (write $e:expr) => {
//         $e.iter_mut()
//     };
//     (read $e:expr, $ent:expr) => {
//         $e.get($ent)
//     };
//     (write $e:expr, $ent:expr) => {
//         $e.get_mut($ent)
//     };

//     ($args:expr,|$pre:tt $name:tt:$type:ty $(,$pres:tt $names:tt:$types:ty),*| $func:block) => {
//         {

//         use std::sync::{MutexGuard,Mutex,RwLockReadGuard,RwLockWriteGuard,Condvar,Arc};
//         use matrix_engine::{systems::SystemArgs,registry::{ComponentRegistry,SafeCollection,TryReadError},components::{ComponentCollection}};

//             let Some(registry) = $args.read_components() else {
//                 panic!();
//             };
//             let (reg_mutex,reg_cv) = registry.get_conditional_mutex();
//             #[allow(unused_mut,unused_variables)]
//             {

//                 match {
//                         fn get<'a>(reg:&'a RwLockReadGuard<ComponentRegistry>) -> Result<(query!($pre, $type,'a),$(query!($pres, $types,'a),)*),TryReadError> {
//                             Ok((query!($pre,reg,$type),$(query!($pres,reg,$types)),*))
//                         }

//                         let mut guard = reg_mutex.lock().unwrap();
//                     let mut state = get(&registry);
//                     while let Err(ref e) = state {
//                         match e{
//                             TryReadError::CantRead |TryReadError::NotExist => break,
//                             _ => {}
//                         }

//                         guard = reg_cv.wait(guard).unwrap();
//                         drop(state);
//                         state = get(&registry);
//                     }
//                     drop(guard);
//                     state
//                 }{
//                     Ok((mut $name,$(mut $names),*))=> {
//                         for (entity,$name) in query!($pre $name) {
//                             let ($($names,)*) = ($(match query!($pres $names,e){
//                                 Some(a)=>a,
//                                 None=>continue,
//                             }),*);
//                             $func
//                         }
//                     },
//                     Err(r) => {
//                         match r {
//                             TryReadError::CantRead => panic!("cant read data!"),
//                             _=>{}
//                         }
//                     },
//                 }

//             };

//             reg_cv.notify_all();

//     }

//     };
// }
>>>>>>> temp
