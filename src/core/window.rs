use std::collections::HashMap;

use glfw::{Context, WindowEvent};

use super::entity::Entity;

pub struct Window {
    window: glfw::PWindow,
    events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
    entity: Entity,
}

impl Window {
    pub fn new(glfw: &mut glfw::Glfw, (width, height): (u32, u32), title: &str) -> Option<Window> {
        let (mut window, events) =
            glfw.create_window(width, height, title, glfw::WindowMode::Windowed)?;
        window.set_all_polling(true);

        Some(Self {
            window,
            events,
            entity: Entity::new(),
        })
    }

    pub fn swap_buffers(&mut self) {
        self.window.swap_buffers();
    }

    pub fn flush_events_iter(&self) -> glfw::FlushedMessages<'_, (f64, WindowEvent)> {
        glfw::flush_messages(&self.events)
    }
    pub fn entity(&self) -> &Entity {
        &self.entity
    }
}

pub struct WindowRegistry(HashMap<Entity, Window>);

impl Default for WindowRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl WindowRegistry {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    pub fn iter(&self) -> impl Iterator<Item = (&'_ Entity, &'_ Window)> {
        self.0.iter()
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&'_ Entity, &'_ mut Window)> {
        self.0.iter_mut()
    }

    pub fn values_mut(&mut self) -> impl Iterator<Item = &'_ mut Window> {
        self.0.values_mut()
    }
    pub fn values(&mut self) -> impl Iterator<Item = &'_ Window> {
        self.0.values()
    }
    pub fn add_window(&mut self, window: Window) {
        self.0.insert(*window.entity(), window);
    }
}
