use super::scene::Scene;

pub trait Plugin {
    fn build(&self,scene:&mut Scene);
}
