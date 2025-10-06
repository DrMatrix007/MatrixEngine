use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop,
    window::WindowId,
};

use crate::engine::{Engine, system_registries::Stage};

impl ApplicationHandler for Engine {
    fn resumed(&mut self, active_event_loop: &ActiveEventLoop) {
        self.run_stages(&[Stage::Startup], active_event_loop);
    }

    fn window_event(
        &mut self,
        active_event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if event == WindowEvent::RedrawRequested {
            self.run_stages(
                &[Stage::PreRender(window_id), Stage::Render(window_id)],
                active_event_loop,
            );
        }
        let stage = Stage::WindowEvent(event);
        self.run_stages(&[stage], active_event_loop);
    }
    fn about_to_wait(&mut self, active_event_loop: &ActiveEventLoop) {
        self.run_stages(
            &[Stage::PreUpdate, Stage::Update, Stage::PostUpdate],
            active_event_loop,
        );
    }
}
