use crate::engine::{events::MatrixEventable, plugins::Plugin};

pub struct RendererPlugin {}

impl<CustomEvents: MatrixEventable> Plugin<CustomEvents> for RendererPlugin {
    fn build(&self, scene: &mut crate::engine::scene::Scene<CustomEvents>) {
        
    }
}
