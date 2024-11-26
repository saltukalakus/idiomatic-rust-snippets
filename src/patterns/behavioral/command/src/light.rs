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
}