use super::scene::Scene;

pub trait Plugin<CustomEvents> {
    fn build(&self,scene:&mut Scene<CustomEvents>);
}
