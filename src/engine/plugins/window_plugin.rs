use winit::window::{Window, WindowAttributes};

use crate::engine::{
    events::{MatrixEvent, MatrixEventable},
    query::{ReadE, ReadR, WriteE, WriteR},
    scene::NonSendEngineStartupArgs,
};

use super::Plugin;

pub struct WindowPlugin {
    name: String,
}

impl WindowPlugin {
    pub fn new(name: impl AsRef<str>) -> Self {
        Self {
            name: name.as_ref().to_string(),
        }
    }
}

impl<CustomEvents: MatrixEventable> Plugin<CustomEvents> for WindowPlugin {
    fn build(&self, scene: &mut crate::engine::scene::Scene<CustomEvents>) {
        scene.add_non_send_startup_system(
            |args: &NonSendEngineStartupArgs, window: &mut WriteR<Window, CustomEvents>| {
                window.insert_and_notify(
                    args.event_loop
                        .create_window(WindowAttributes::default())
                        .unwrap(),
                )
            },
        );

        scene.add_send_system(|window: &mut ReadR<Window>| {
            if let Some(window) = window.get() {
                window.request_redraw();
            }
        });
        scene.add_send_system(
            move |events: &mut ReadE<CustomEvents>, event_writer: &mut WriteE<CustomEvents>| {
                if events.close_requested() {
                    event_writer.send(MatrixEvent::Exit).unwrap();
                }
            },
        );
    }
}
