use std::{
    collections::VecDeque,
    sync::{atomic::AtomicBool, Arc},
};

use crate::{
    components::ComponentRegistry,
    dispatchers::{Dispatcher, IntoDispatcher},
    systems::StartupSystem,
    thread_pool::ThreadPool,
};

#[derive(Default)]
pub struct Scene {
    components: ComponentRegistry,
    startups: VecDeque<Box<dyn Dispatcher<DispatchArgs = Scene>>>,
}

impl Scene {
    pub fn add_startup(
        &mut self,
        sys: impl Dispatcher<DispatchArgs = Scene> + 'static,
    ) -> &mut Self
where {
        self.startups.push_back(Box::new(sys));
        self
    }
    pub fn get_component_registry(&mut self) -> &mut ComponentRegistry {
        &mut self.components
    }
    pub(crate) fn setup(&mut self) {
        let startups = self.startups.drain(..).collect::<Vec<_>>();
        for mut i in startups {
            unsafe {
                i.dispatch(self);
            };
        }
    }
    pub(crate) fn update(&mut self, _: &SceneUpdateArgs) {}
}

pub struct SceneUpdateArgs<'a> {
    pub quit: Arc<AtomicBool>,
    pub pool: &'a ThreadPool<()>,
}
