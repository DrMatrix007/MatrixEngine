// use std::{collections::VecDeque, io};

// use crate::{
//     dispatchers::systems::UnsafeBoxedDispatcher,
//     scene::{Scene, SceneUpdateArgs},
// };

// use super::{
//     access::Access,
//     thread_pool::{Job, ThreadPool, ThreadPoolSender},
// };

use crate::scene::{Scene, SceneUpdateArgs};

pub trait Scheduler {
    fn run(
        &mut self,
        scene: &mut Scene,
        args: &SceneUpdateArgs,
    );
}

// pub struct SingleThreadScheduler;

// impl Scheduler for SingleThreadScheduler {
//     fn run(
//         &mut self,
//         scene: &mut Scene,
//         _: &SceneUpdateArgs,
//     ) {
//         for i in scene.iter_mut() {
//             unsafe { i.as_mut().dispatch(scene) };
//         }
//     }
//     // fn run_group(&mut self,dis:Vec<UnsafeBoxedDispatcher>)
// }


// pub struct MultiThreadedScheduler {
//     pool: ThreadPool<UnsafeBoxedDispatcher>,
//     done: VecDeque<UnsafeBoxedDispatcher>,
//     pending: VecDeque<UnsafeBoxedDispatcher>,
//     access_state: Access,
// }

// impl MultiThreadedScheduler {
//     pub fn new(thread_count: usize) -> Self {
//         Self {
//             pool: ThreadPool::new(thread_count),
//             done: Default::default(),
//             pending: Default::default(),
//             access_state: Default::default(),
//         }
//     }
//     pub fn with_amount_of_cores() -> io::Result<Self> {
//         Ok(Self::new(std::thread::available_parallelism()?.get()))
//     }
//     fn try_run_dispatcher(
//         pool: &mut ThreadPoolSender<UnsafeBoxedDispatcher>,
//         access_state: &mut Access,
//         mut dis: UnsafeBoxedDispatcher,
//         pending: &mut VecDeque<UnsafeBoxedDispatcher>,
//         scene: &mut Scene,
//     ) {
//         if let Ok(()) = access_state.try_combine(dis.as_access()) {
//             let data = unsafe { dis.as_mut().dispatch(scene) };

//             pool.send(Job::Work(Box::new(move || {
//                 dis.get_mut()
//                     .try_run(data)
//                     .expect("this function should work");
//                 dis
//             })))
//             .expect("this should work");
//         } else {
//             pending.push_back(dis);
//         }
//     }
// }

// impl Scheduler for MultiThreadedScheduler {
//     fn run(
//         &mut self,
//         scene: &mut Scene,
//         _args: &SceneUpdateArgs,
//     ) {
//         let mut sender = self.pool.sender();
//         self.access_state.clear();

//         while let Some(dis) = scene.pop() {
//             Self::try_run_dispatcher(
//                 &mut sender,
//                 &mut self.access_state,
//                 dis,
//                 &mut self.pending,
//                 scene,
//             );

//             for dis in self.pool.try_recv_iter() {
//                 self.access_state.remove(dis.as_access());
//                 self.done.push_back(dis);
//             }
//         }
//         for dis in self.pool.recv_iter() {
//             self.access_state.remove(dis.as_access());
//             self.done.push_back(dis);

//             for _ in 0..self.pending.len() {
//                 let dis = self
//                     .pending
//                     .pop_back()
//                     .expect("there should be a value here");
//                 Self::try_run_dispatcher(
//                     &mut sender,
//                     &mut self.access_state,
//                     dis,
//                     &mut self.pending,
//                     scene,
//                 );
//             }
//         }

//         for i in self.done.drain(..) {
//             dispatchers.push(i);
//         }
//     }
// }
