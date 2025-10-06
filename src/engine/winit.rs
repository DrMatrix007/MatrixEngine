use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop,
    window::WindowId,
};

use crate::engine::Engine;

impl ApplicationHandler for Engine {
    fn resumed(&mut self, active_event_loop: &ActiveEventLoop) {
        self.startup(active_event_loop);
    }

    fn window_event(
        &mut self,
        active_event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if event == WindowEvent::RedrawRequested {
            self.frame_render(&window_id, active_event_loop);
        }
    }
    fn about_to_wait(&mut self, active_event_loop: &ActiveEventLoop) {
        self.frame_update(active_event_loop);
    }
}
