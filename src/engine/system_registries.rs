use std::{collections::VecDeque, mem::swap};

use winit::{event::WindowEvent, window::WindowId};

use crate::engine::systems::System;

#[derive(Debug, Clone)]
pub enum Stage {
    PreUpdate,
    Update,
    PostUpdate,
    PreRender(WindowId),
    Render(WindowId),
    WindowEvent(WindowEvent),

    Startup,
}
impl Stage {
    pub fn to_descriptor(&self) -> StageDescriptor {
        match self {
            Stage::PreUpdate => StageDescriptor::PreUpdate,
            Stage::Update => StageDescriptor::Update,
            Stage::PostUpdate => StageDescriptor::PostUpdate,
            Stage::PreRender(_) => StageDescriptor::PreRender,
            Stage::Render(_) => StageDescriptor::Render,
            Stage::WindowEvent(_) => StageDescriptor::WindowEvent,
            Stage::Startup => StageDescriptor::Startup,
        }
    }
}

pub enum StageDescriptor {
    PreUpdate,
    Update,
    PostUpdate,
    PreRender,
    Render,
    WindowEvent,

    Startup,
}

#[derive(Debug)]
pub enum SystemRegistryNextFrameError {
    NotAllSystemExhausted,
}

type BoxedSystem<Registry> = Box<dyn System<Registry = Registry>>;

pub struct SystemCollection<Registry> {
    systems: VecDeque<BoxedSystem<Registry>>,
    systems_done: VecDeque<BoxedSystem<Registry>>,
}

impl<Registry> Default for SystemCollection<Registry> {
    fn default() -> Self {
        Self {
            systems: Default::default(),
            systems_done: Default::default(),
        }
    }
}

impl<Registry> SystemCollection<Registry> {
    pub fn add_system(&mut self, system: impl System<Registry = Registry> + 'static) {
        self.systems.push_back(Box::new(system));
    }

    pub fn take_out_system(&mut self) -> Option<BoxedSystem<Registry>> {
        self.systems.pop_front()
    }

    pub fn take_back_system(&mut self, b: BoxedSystem<Registry>) {
        self.systems_done.push_back(b);
    }

    pub fn prepare_next_frame(&mut self) -> Result<(), SystemRegistryNextFrameError> {
        if !self.systems.is_empty() {
            return Err(SystemRegistryNextFrameError::NotAllSystemExhausted);
        }
        swap(&mut self.systems, &mut self.systems_done);
        Ok(())
    }
}

pub struct SystemRegistry<Registry> {
    startup_systems: SystemCollection<Registry>,
    pre_update_systems: SystemCollection<Registry>,
    update_systems: SystemCollection<Registry>,
    post_update_systems: SystemCollection<Registry>,
    pre_render_systems: SystemCollection<Registry>,
    render_systems: SystemCollection<Registry>,
    window_event_systems: SystemCollection<Registry>,
}

impl<Registry> Default for SystemRegistry<Registry> {
    fn default() -> Self {
        Self {
            startup_systems: Default::default(),
            pre_update_systems: Default::default(),
            update_systems: Default::default(),
            post_update_systems: Default::default(),
            pre_render_systems: Default::default(),
            render_systems: Default::default(),
            window_event_systems: Default::default()
        }
    }
}

impl<Registry> SystemRegistry<Registry> {
    pub fn startup_systems_mut(&mut self) -> &mut SystemCollection<Registry> {
        &mut self.startup_systems
    }

    pub fn pre_update_systems_mut(&mut self) -> &mut SystemCollection<Registry> {
        &mut self.pre_update_systems
    }

    pub fn update_systems_mut(&mut self) -> &mut SystemCollection<Registry> {
        &mut self.update_systems
    }

    pub fn post_update_systems_mut(&mut self) -> &mut SystemCollection<Registry> {
        &mut self.post_update_systems
    }

    pub fn add_system(
        &mut self,
        stage: StageDescriptor,
        system: impl System<Registry = Registry> + 'static,
    ) {
        self.get_system_collection(&stage).add_system(system);
    }
    pub fn get_system_collection(
        &mut self,
        stage: &StageDescriptor,
    ) -> &mut SystemCollection<Registry> {
        match stage {
            StageDescriptor::PreUpdate => &mut self.pre_update_systems,
            StageDescriptor::Update => &mut self.update_systems,
            StageDescriptor::PostUpdate => &mut self.post_update_systems,
            StageDescriptor::PreRender => &mut self.pre_render_systems,
            StageDescriptor::Render => &mut self.render_systems,
            StageDescriptor::WindowEvent => &mut self.window_event_systems,
            StageDescriptor::Startup => &mut self.startup_systems,

        }
    }
}
