use crate::observer::Observer;

pub struct Subject {
    observers: Vec<Box<dyn Observer>>,
}

impl Subject {
    pub fn new() -> Self {
        Subject {
            observers: Vec::new(),
        }
    }

    pub fn add_observer(&mut self, observer: Box<dyn Observer>) {
        self.observers.push(observer);
    }

    pub fn notify_observers(&self, message: &str) {
        for observer in &self.observers {
            observer.update(message);
        }
    }
}