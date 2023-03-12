use std::{
    any::TypeId,
    cell::{RefCell, RefMut, Ref},
    collections::{HashSet, hash_map},
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        mpsc::{SendError, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant}, borrow::{BorrowMut, Borrow},
};

use crate::{
    components::{Component, ComponentCollection},
    entity::Entity,
    queries::query::{Action, Query, QueryData, QueryRequest, QueryResult},
    server_client::{Client, Request, Response},
};

pub struct SystemArgs {
    quit: Arc<AtomicBool>,
    conn: Client<QueryRequest, QueryResult>,
}
pub struct QueryCollection {
    data: QueryResult,
    server: Sender<Request<QueryRequest, QueryResult>>,
    clinet_sender: Sender<Response<QueryResult>>,
}

pub trait RefType {
    type NonRef;
    type Collection<'a>;
    
}
impl<T: Component+'static> RefType for &T {
    type NonRef = T;

    type Collection<'a>
    
    = &'a ComponentCollection<T> where T: 'a;

    
}

impl<T: Component+'static> RefType for &mut T {
    type NonRef = T;
    
    type Collection<'a>
    
    = RefMut<'a, ComponentCollection<T>> where T: 'a;
}

pub trait ComponentRefType<'a> {
    type Item<'b>;
    type Iter : Iterator<Item = (&'a Entity,Self::Item<'a>)>;
    fn iter_comps(&'a mut self)-> Self::Iter;
    fn get_comp(& mut self,e:&Entity) -> Option<Self::Item<'_>>;


}

impl<'a,T:'static+Component> ComponentRefType<'a> for Ref<'a ,ComponentCollection<T>> {
    type Item<'b> = &'b T;
    type Iter = hash_map::Iter<'a ,Entity,T>;

    fn iter_comps(&'a  mut self)-> Self::Iter{
        self.iter()
    }
    fn get_comp(& mut self,e:&Entity) -> Option<Self::Item<'_>> {
        self.get(e)
    }

}
impl<'a,T:'static+Component> ComponentRefType<'a> for RefMut<'a,ComponentCollection<T>> {
    type Item<'b> = &'b mut T;
    type Iter = hash_map::IterMut<'a, Entity,T>;

    fn iter_comps(&'a mut self) -> Self::Iter {
        (self).iter_mut()
    }
    fn get_comp(& mut self,e:&Entity) -> Option<Self::Item<'_>> {
        self.get_mut(e)
    }

}



impl QueryCollection {
    pub fn finish(self) -> Result<(), SendError<Request<QueryRequest, QueryResult>>> {
        if let QueryResult::Ok { data } = self.data {
            self.server.send(Request::new(
                QueryRequest::QueryDone(data),
                self.clinet_sender.clone(),
            ))
        } else {
            Ok(())
        }
    }

    // pub fn iter_ref<T:Component+'static>(&self) -> impl Iterator<Item = (&Entity,&T)> {
    //     if let QueryResult::Ok { data } = &self.data {
    //         if let Some(e) = data.data.get(&TypeId::of::<T>()) {
    //             if let Action::Read(data) = e {
    //                 return (*data).get_mut().as_any().downcast_ref::<ComponentCollection<T>>().unwrap().iter();
    //             }
    //         }
    //     } 
    //     return std::collections::hash_map::HashMap::new().iter();
    // }
    

}

// type ComponentRefTypeIter<'a,T> = <<T as GroupFetch<'a>>::First as ComponentRefType<'a>>::Iter;
// type ComponentRefTypeItem<'a,T> = <<T as GroupFetch<'a>>::First as ComponentRefType<'a>>::Item<'a>;

// trait GroupFetch<'a> {
//     type First: ComponentRefType<'a>+'a;
//     type Rest:'a;


//     fn get_data(data:&'a mut QueryData) -> QueryIter<Self::First,Self::Rest>;
//     // fn get(&mut self, e:&Entity) -> Option<Self::Result>;
// }


// macro_rules! impl_iter {
//     ($name:ident $type:ident $(,$names:tt $types:ident)* $(,)?) => {

//         // impl<'a, $($types:ComponentRefType<'a>+'a),*> GroupFetch<'a> for ($($types),*) {
//         //     type Result = ($($types::Item<'a>),* );
//         // fn get(&mut self, e:&Entity) -> Option<Self::Result> 
//         // {
//         //     let ($($names),*) = self;
//         //     let ($($names),*) = ($($names.get_comp(e)?),*);
//         //     Some(($($names),*))

//         // }

//         // }

//         impl<'a,$type:ComponentRefType<'a>+'a,$($types:ComponentRefType<'a>+'a),*> GroupFetch<'a> for ($type, $($types),*){
//             // type Item=(&'a Entity, $type::Item<'a>,$($types::Item<'a>),*);
//             type First= $type;
//             type Rest= ($($types,)*);
//             fn get_data(data:&'a mut QueryData) -> Option<QueryIter<Self::First,Self::Rest>>  {
//                 QueryIter {
//                     first: 
//                 }
//             }
            
            

//         }
//         impl<'a,$type:ComponentRefType<'a>+'a,$($types:ComponentRefType<'a>+'a),*> QueryIter<'a, $type,($($types),*)> {
//             // type Item=(&'a Entity, $type::Item<'a>,$($types::Item<'a>),*);
//             fn iterate<F:FnMut(&Entity,$type::Item<'_>, $($types::Item<'_>),*)>(&mut self,mut f:F) {
//                 if let Some((e,c)) = self.first.next() {
//                     let ($($names),*) = &mut self.rest; 
//                     // ;
//                     if let ($(Some($names)),*) = ($(($names).get_comp(e)),*) {
//                         // return Some((e,c $(,$names)*));
//                         f(e,c,$($names),*);
//                     }
//                 };

//             }
            

//         }
//     };
// }



// impl_iter!(a A, b B, c C, d D);

macro_rules! iterate_query {
    ($q:expr,||) => {
        
    };
}


impl SystemArgs {
    pub fn new(quit: Arc<AtomicBool>, sender: Sender<Request<QueryRequest, QueryResult>>) -> Self {
        Self {
            quit,
            conn: Client::new(sender),
        }
    }

    pub fn stop(&self) {
        self.quit.store(true, Ordering::Relaxed);
    }
    pub fn query<T>(&self, m: impl Iterator<Item = T>) -> Option<QueryCollection>
    where
        HashSet<Action<TypeId>>: FromIterator<T>,
    {
        let map = m.collect();
        self.conn
            .send(QueryRequest::Query(Query { data: map }))
            .ok()?;

        self.conn.recv().ok().map(|x| QueryCollection {
            data: x.unpack(),
            server: self.conn.server_sender(),
            clinet_sender: self.conn.sender(),
        })
    }
}

pub trait System {
    fn update(&mut self, args: &mut SystemArgs);
}

pub(crate) fn spawn_system(
    sys: SystemCreator,
    target_fps: Arc<AtomicU64>,
    quit: Arc<AtomicBool>,
    sender: Sender<Request<QueryRequest, QueryResult>>,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut sys = sys.create();
        let mut target;
        let mut fps;
        let mut start = Instant::now();
        let mut len;
        let quit = quit.clone();
        let mut args = SystemArgs::new(quit.clone(), sender);
        while !quit.load(std::sync::atomic::Ordering::Relaxed) {
            sys.update(&mut args);
            fps = 1.0 / target_fps.load(Ordering::Relaxed) as f64;
            if fps.is_finite() {
                target = Duration::from_secs_f64(fps);
                len = Instant::now() - start;
                if len < target {
                    spin_sleep::sleep(target - len);
                }
                start = Instant::now();
            }
        }
    })
}

pub struct SystemCreator {
    creator: Box<dyn FnOnce() -> Box<dyn System> + Send + Sync>,
}

impl SystemCreator {
    pub fn default_function<T: System + Default + 'static>() -> Self {
        Self {
            creator: Box::new(|| Box::<T>::default()),
        }
    }
    pub fn with_function(f: impl FnOnce() -> Box<dyn System> + Send + Sync + 'static) -> Self {
        Self {
            creator: Box::new(f),
        }
    }

    pub fn create(self) -> Box<dyn System> {
        (self.creator)()
    }
}
