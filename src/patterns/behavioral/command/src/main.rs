mod command;
mod concrete_command;
mod light;
mod remote_control;

use concrete_command::TurnOnCommand;
use concrete_command::TurnOffCommand;
use remote_control::RemoteControl;
use light::Light;

fn main() {
    let mut light = Light::new();
    {
        let turn_on_command = TurnOnCommand { light: &mut light };
        let mut remote = RemoteControl::new();
        remote.set_command(Box::new(turn_on_command));
        remote.press_button();
    }

    {
        let turn_off_command = TurnOffCommand { light: &mut light };
        let mut remote = RemoteControl::new();
        remote.set_command(Box::new(turn_off_command));
        remote.press_button();
    }
}