pub mod window_plugin;

use super::{events::MatrixEventable, scene::Scene};

pub trait Plugin<CustomEvents: MatrixEventable> {
    fn build(&self, scene: &mut Scene<CustomEvents>);
}
