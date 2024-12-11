// Observer type
pub trait Observer {
    fn update(&self, message: &str);
}

// Concrete observer
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

// Subject
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

fn main() {
    let mut subject = Subject::new();

    let observer1 = ConcreteObserver::new("Observer 1");
    let observer2 = ConcreteObserver::new("Observer 2");

    subject.add_observer(Box::new(observer1));
    subject.add_observer(Box::new(observer2));

    subject.notify_observers("Hello, observers!");
}