use crate::engine::{events::MatrixEventable, plugins::Plugin};

use super::renderer_systems::{create_renderer_resource, handle_resize, renderer_system};

pub struct RendererPlugin;

impl<CustomEvents: MatrixEventable> Plugin<CustomEvents> for RendererPlugin {
    fn build(&self, scene: &mut crate::engine::scene::Scene<CustomEvents>) {
        scene.add_non_send_system(create_renderer_resource);
        scene.add_send_system(handle_resize);
        scene.add_send_system(renderer_system);
        
    }
}
