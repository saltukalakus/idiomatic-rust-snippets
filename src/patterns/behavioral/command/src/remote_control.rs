use crate::command::Command; 

pub struct RemoteControl<'a> {
    command: Option<Box<dyn Command + 'a>>,
}

impl<'a> RemoteControl<'a> {
    pub fn new() -> Self {
        RemoteControl { command: None }
    }

    pub fn set_command(&mut self, command: Box<dyn Command + 'a>) {
        self.command = Some(command);
    }

    pub fn press_button(&mut self) {
        if let Some(ref mut command) = self.command {
            command.execute();
        }
    }
}