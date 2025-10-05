use winit::{application::ApplicationHandler, event::WindowEvent};

use crate::engine::Engine;

impl ApplicationHandler for Engine {
    fn resumed(&mut self, _: &winit::event_loop::ActiveEventLoop) {
        self.startup();
    }
    
    fn window_event(
        &mut self,
        _: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if event == WindowEvent::RedrawRequested {
            self.frame_render(&window_id);
        }
    }
    fn about_to_wait(&mut self, _: &winit::event_loop::ActiveEventLoop) {
        self.frame_update();
    }
}
