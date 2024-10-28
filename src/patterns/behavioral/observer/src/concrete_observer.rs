use crate::observer::Observer;

pub struct ConcreteObserver {
    name: String,
}

impl ConcreteObserver {
    pub fn new(name: &str) -> Self {
        ConcreteObserver {
            name: name.to_string(),
        }
    }
}

impl Observer for ConcreteObserver {
    fn update(&self, message: &str) {
        println!("{} received message: {}", self.name, message);
    }
}