use crate::{
    dispatchers::Dispatcher,
    scene::{Scene, SceneUpdateArgs},
};

pub trait Scheduler {
    fn run(
        &mut self,
        dispatchers: &mut Vec<Box<dyn Dispatcher<DispatchArgs = Scene>>>,
        scene: &mut Scene,
        args: &SceneUpdateArgs,
    );
}

pub struct SingleThreadScheduler;

impl Scheduler for SingleThreadScheduler {
    fn run(
        &mut self,
        dispatchers: &mut Vec<Box<dyn Dispatcher<DispatchArgs = Scene>>>,
        scene: &mut Scene,
        _: &SceneUpdateArgs,
    ) {
        for i in dispatchers.iter_mut() {
            unsafe { i.dispatch(scene) };
        }
    }

}
