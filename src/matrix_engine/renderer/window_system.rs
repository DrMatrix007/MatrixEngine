use std::time::Instant;

use sfml::{
    self,
    graphics::{Color, RenderTarget, RenderWindow},
    window::{ContextSettings, Event, Style, VideoMode},
};

use crate::matrix_engine::systems::System;
pub struct WindowSystem {
    window: RenderWindow,
    avg_fps: f64,
    fps_counter: u128,
    last_frame: Instant,
}

impl Default for WindowSystem {
    fn default() -> Self {
        Self::new(RenderWindow::new(
            VideoMode::new(1000, 500, 32),
            "matrix_engine",
            Style::CLOSE | Style::TITLEBAR | Style::RESIZE,
            &ContextSettings::default(),
        ))
    }
}

impl WindowSystem {
    pub fn new(window: RenderWindow) -> Self {
        Self {
            window,
            avg_fps: 0.0,
            fps_counter: 0,
            last_frame: Instant::now(),
        }
    }
}
unsafe impl Send for WindowSystem {}
impl System for WindowSystem {
    fn update(&mut self, args: crate::matrix_engine::systems::SystemArgs) {
        

        self.window.clear(Color::CYAN);
        self.window.display();

        while let Some(event) = self.window.poll_event() {
            match event {
                Event::Closed => {
                    println!("avg: {}",self.avg_fps);
                    args.stop();
                }
                _ => {}
            }
        }
    }
}
