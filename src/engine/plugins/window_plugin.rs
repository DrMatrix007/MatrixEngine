use winit::window::Window;

use crate::engine::{
    events::{MatrixEvent, MatrixEventable},
    query::{ReadE, ReadR, WriteE, WriteR},
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
        scene.add_send_startup_system(|window: &mut WriteR<Window>| {});

        scene.add_send_system(|window: &mut ReadR<Window>| {
            if let Some(window) = window.get() {
                window.request_redraw();
            }
        });
        scene.add_send_system(
            move |(events, event_writer, test): &mut (
                ReadE<CustomEvents>,
                WriteE<CustomEvents>,
                WriteR<i32>,
            )| {
                *test.get_mut().unwrap() = 10;
                if events.close_requested() {
                    event_writer.send(MatrixEvent::Exit).unwrap();
                }
            },
        );
    }
}
