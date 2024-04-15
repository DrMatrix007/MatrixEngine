use std::{borrow::BorrowMut, ops::DerefMut};

use glfw::Glfw;

use crate::core::{
    plugins::Plugin,
    systems::{QueryData, Queryable, ReadNonSendR, WriteNonSendR},
};

use super::window::Window;

pub struct GlfwWindowPlugin {
    window_title: String,
    size: (u32, u32),
}

impl GlfwWindowPlugin {
    pub fn new(window_title: String, (width, height): (u32, u32)) -> Self {
        Self {
            window_title,
            size: (width, height),
        }
    }
}

impl Plugin for GlfwWindowPlugin {
    fn build(&self, scene: &mut crate::core::scene::Scene) {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
        let window = Window::new(&mut glfw, self.size, &self.window_title);

        scene.registry_mut().resources.add_resource(window);
        scene.registry_mut().resources.add_resource(glfw);

        scene.add_system(|mut glfw:QueryData<WriteNonSendR<Glfw>>|{
            if let Some(glfw) = glfw.as_mut() {
                glfw.poll_events();
            }
        });
    }
}
