pub mod window_plugin;

use super::scene::Scene;

pub trait Plugin<CustomEvents:Send> {
    fn build(&self,scene:&mut Scene<CustomEvents>);
}
