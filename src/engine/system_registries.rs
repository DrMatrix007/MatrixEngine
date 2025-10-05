use std::{
    collections::{HashMap, VecDeque},
    mem::swap,
};

use winit::window::WindowId;

use crate::engine::systems::System;

#[derive(Debug, Clone, Copy)]
#[repr(usize)]
pub enum Stage {
    PreUpdate = 0,
    Update,
    PostUpdate,
    PreRender(WindowId),
    Render(WindowId),

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
    pre_render_systems: HashMap<WindowId, SystemCollection<Registry>>,
    render_systems: HashMap<WindowId, SystemCollection<Registry>>,
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

    pub fn render_systems_mut(&mut self, id: &WindowId) -> &mut SystemCollection<Registry> {
        self.render_systems.entry(*id).or_default()
    }

    pub fn pre_render_systems_mut(&mut self, id: &WindowId) -> &mut SystemCollection<Registry> {
        self.pre_render_systems.entry(*id).or_default()
    }

    pub fn add_system(&mut self, stage: Stage, system: impl System<Registry = Registry> + 'static) {
        self.get_system_collection(&stage).add_system(system);
    }
    pub fn get_system_collection(&mut self, stage: &Stage) -> &mut SystemCollection<Registry> {
        match stage {
            Stage::PreUpdate => &mut self.pre_update_systems,
            Stage::Update => &mut self.update_systems,
            Stage::PostUpdate => &mut self.post_update_systems,
            Stage::PreRender(window_id) => self.pre_render_systems_mut(window_id),
            Stage::Render(window_id) => self.render_systems_mut(window_id),
            Stage::Startup => &mut self.startup_systems,
        }
    }
}
