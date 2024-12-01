use std::cell::RefCell;
use std::rc::Rc;

// Command
pub trait Command {
    fn execute(&self);
}

// ConcreteCommand for turning on the light
pub struct TurnOnCommand {
    pub light: Rc<RefCell<Light>>,
}

impl Command for TurnOnCommand {
    fn execute(&self) {
        self.light.borrow_mut().turn_on();
    }
}

// ConcreteCommand for turning off the light
pub struct TurnOffCommand {
    pub light: Rc<RefCell<Light>>,
}

impl Command for TurnOffCommand {
    fn execute(&self) {
        self.light.borrow_mut().turn_off();
    }
}

// Invoker
pub struct RemoteControl{
    command: Option<Box<dyn Command>>,
}

impl RemoteControl {
    pub fn new() -> Self {
        RemoteControl { command: None }
    }

    pub fn set_command(&mut self, command: Box<dyn Command >) {
        self.command = Some(command);
    }

    pub fn press_button(&mut self) {
        if let Some(ref mut command) = self.command {
            command.execute();
        }
    }
}

// Receiver
pub struct Light {
    is_on: bool,
}

impl Light {
    pub fn new() -> Self {
        Light { is_on: false }
    }

    pub fn turn_on(&mut self) {
        self.is_on = true;
        println!("The light is on");
    }

    pub fn turn_off(&mut self) {
        self.is_on = false;
        println!("The light is off");
    }

    pub fn is_on(&self) -> bool {
        self.is_on
    }
}

// Client
fn main() {
    let light = Rc::new(RefCell::new(Light::new()));
    let turn_on_command = TurnOnCommand { light: Rc::clone(&light) };
    let turn_off_command = TurnOffCommand { light: Rc::clone(&light) };
    let mut remote = RemoteControl::new();


    remote.set_command(Box::new(turn_on_command));
    remote.press_button();
    println!("Light Status: {}", light.borrow().is_on()); // Borrow the Light instance


    remote.set_command(Box::new(turn_off_command));
    remote.press_button();
    println!("Light Status: {}", light.borrow().is_on()); // Borrow the Light instance

}