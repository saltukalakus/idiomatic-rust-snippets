use crate::light::Light;
use crate::command::Command; 


pub struct TurnOnCommand<'a> {
    pub light: &'a mut Light,
}

impl<'a> Command for TurnOnCommand<'a> {
    fn execute(&mut self) {
        self.light.turn_on();
    }
}

// ConcreteCommand for turning off the light
pub struct TurnOffCommand<'a> {
    pub light: &'a mut Light,
}

impl<'a> Command for TurnOffCommand<'a> {
    fn execute(&mut self) {
        self.light.turn_off();
    }
}
