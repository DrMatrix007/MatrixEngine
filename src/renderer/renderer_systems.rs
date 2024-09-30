use std::ops::Deref;

use wgpu::{Device, Queue, Surface};
use winit::window::{Window, WindowId};

use crate::engine::{
    events::MatrixEventable,
    query::{ReadR, WriteR},
    scene::SceneRegistryRefs,
    systems::QuerySystem,
};

pub struct RendererResource {
    device: Device,
    queue: Queue,
    current_window_id: Option<WindowId>,
    surface: Option<Surface<'static>>,
}

pub fn create_renderer_resource(ren: (&mut WriteR<RendererResource>,)) {}

pub struct RendererSystem {}
impl RendererSystem {
    fn create_surface(&mut self) {}
}

impl<T, CustomEvents: MatrixEventable> QuerySystem<SceneRegistryRefs<CustomEvents>, T>
    for RendererSystem
{
    type Query = (ReadR<Window>,);

    fn run(&mut self, _: &mut T, (window,): &mut Self::Query) {
        let window = if let Some(window) = window.get() {
            window
        } else {
            return;
        };
        self.create_surface();
    }
}
