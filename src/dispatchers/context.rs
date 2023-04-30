use std::{
    any::TypeId,
    sync::{
        atomic::{AtomicBool, AtomicU64},
        Arc,
    },
};

use crate::{
    components::resources::{Resource, ResourceHolder},
    events::matrix_event::{MatrixEvent, MatrixEventSender},
    scenes::scene::Scene,
};

#[derive(Clone)]
pub struct Context {
    pub(crate) quit: Arc<AtomicBool>,
    pub(crate) fps: Arc<AtomicU64>,
    pub(crate) sender: MatrixEventSender,
}

impl Context {
    pub fn new(quit: Arc<AtomicBool>, fps: Arc<AtomicU64>, sender: MatrixEventSender) -> Self {
        Self { quit, fps, sender }
    }

    pub fn stop(&self) {
        self.quit.store(true, std::sync::atomic::Ordering::Relaxed);
    }
    pub(crate) fn send_event(
        &self,
        e: MatrixEvent,
    ) -> Result<(), std::sync::mpsc::SendError<MatrixEvent>> {
        self.sender.send(e)
    }
}

pub trait SceneCreator {
    fn create_scene(&self) -> Scene;
}
impl SceneCreator for Context {
    fn create_scene(&self) -> Scene {
        Scene::empty(self.clone())
    }
}

pub trait ResourceHolderManager {
    fn get_or_insert_resource<'a, T: Resource + 'static>(
        &self,
        r: &'a mut ResourceHolder<T>,
        data: T,
    ) -> &'a mut T;

    fn get_or_insert_resource_with<'a, T: Resource + 'static>(
        &self,
        r: &'a mut ResourceHolder<T>,
        data: impl FnOnce() -> T,
    ) -> &'a mut T;

    fn clear_resource<T: Resource + 'static>(&self, r: &mut ResourceHolder<T>);
}

impl ResourceHolderManager for Context {
    fn get_or_insert_resource<'a, T: Resource + 'static>(
        &self,
        r: &'a mut ResourceHolder<T>,
        data: T,
    ) -> &'a mut T {
        self.send_event(MatrixEvent::CreatedResource(TypeId::of::<T>()))
            .expect("the receiver should exist");

        r.get_or_insert(data)
    }
    fn get_or_insert_resource_with<'a, T: Resource + 'static>(
        &self,
        r: &'a mut ResourceHolder<T>,
        data: impl FnOnce() -> T,
    ) -> &'a mut T {
        self.send_event(MatrixEvent::CreatedResource(TypeId::of::<T>()))
            .expect("the receiver should exist");
        r.get_or_insert_with(data)
    }
    fn clear_resource<T: Resource + 'static>(&self, r: &mut ResourceHolder<T>) {
        self.send_event(MatrixEvent::RemovedResource(TypeId::of::<T>()))
            .expect("the receiver should exist");
        r.clear();
    }
}
