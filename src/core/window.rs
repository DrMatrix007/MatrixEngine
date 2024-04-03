pub struct Window {
    window: glfw::Window
}

impl Window {
    pub fn new(glfw:&mut glfw::Glfw, (width,height):(u32,u32),title: &str) -> Option<Window>{
        let (mut window,events) = glfw.create_window(width, height, title, glfw::WindowMode::Windowed)?;
        window.set_all_polling(true);

        Ok(Self {
                    window: window.
                })
    }
}
