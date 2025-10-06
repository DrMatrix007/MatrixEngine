use winit::window::Window;

use crate::engine::{query::Res, system_registries::Stage};

pub fn matrix_renderer(stage: &mut Stage, window: &mut Res<Window>) {
    let window = match (stage, window.as_mut()) {
        (Stage::Render(id), maybe_window) => {
            if let Some(window) = maybe_window {
                if *id != window.id() {
                    return;
                }else {
                    window
                }
            }else {
                return;
            }
        }
        _ => {
            panic!("this should be run in StageDescriptor::Render!");
        }
    };

    println!("render");



    window.request_redraw();
}
