use winit::window::WindowAttributes;

use crate::engine::{
    EngineState,
    commands::{Command, CommandError},
};

pub struct AddWindowResourceCommand {
    attrs: WindowAttributes,
}

impl AddWindowResourceCommand {
    pub fn new(attrs: WindowAttributes) -> Self {
        Self { attrs }
    }
}

impl Command for AddWindowResourceCommand {
    type Args = EngineState;

    fn run(&mut self, data: &mut Self::Args) -> Result<(), super::CommandError> {
        let window = data
            .active_event_loop()
            .create_window(self.attrs.clone())
            .map_err(|_| CommandError::UnknownError)?;

        data.resources
            .insert(window)
            .map_err(CommandError::LockableError)?;

        Ok(())
    }
}
