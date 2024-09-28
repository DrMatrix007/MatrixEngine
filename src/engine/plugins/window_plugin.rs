use winit::window::{Window, WindowAttributes};

use crate::engine::{
    entity::Entity,
    query::{ReadC, WriteC},
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

impl<T> Plugin<T> for WindowPlugin {
    fn build(&self, scene: &mut crate::engine::scene::Scene<T>) {
        let title = self.name.to_string();
        scene.add_non_send_startup_system(
            move |args: &mut NonSendEngineStartupArgs, data: &mut WriteC<Window>| {
                data.insert(
                    Entity::new(),
                    args.event_loop
                        .create_window(WindowAttributes::default().with_title(&title))
                        .unwrap(),
                );
            },
        );

        scene.add_send_system(|data: &mut ReadC<Window>| {
            for (_, w) in data.iter() {
                w.request_redraw();
            }
            println!("update")
        });
    }
}
