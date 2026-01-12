use std::sync::Arc;
use std::sync::mpsc::{channel, Sender};

// Approach 1: Traditional Observer with proper ownership
// Uses weak references to allow unsubscribing

pub trait Observer: Send + Sync {
    fn update(&self, message: &str);
}

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
        println!("{} received: {}", self.name, message);
    }
}

pub struct Subject {
    // Use Arc for shared ownership - observers can be removed
    observers: Vec<Arc<dyn Observer>>,
}

impl Subject {
    pub fn new() -> Self {
        Subject {
            observers: Vec::new(),
        }
    }

    pub fn subscribe(&mut self, observer: Arc<dyn Observer>) {
        self.observers.push(observer);
    }

    pub fn unsubscribe(&mut self, observer: &Arc<dyn Observer>) {
        self.observers.retain(|o| !Arc::ptr_eq(o, observer));
    }

    pub fn notify(&self, message: &str) {
        for observer in &self.observers {
            observer.update(message);
        }
    }
}

// Approach 2: Idiomatic Rust using channels (preferred)
// This is more flexible and follows Rust's message-passing philosophy

struct ChannelSubject {
    subscribers: Vec<Sender<String>>,
}

impl ChannelSubject {
    fn new() -> Self {
        ChannelSubject {
            subscribers: Vec::new(),
        }
    }

    fn subscribe(&mut self) -> std::sync::mpsc::Receiver<String> {
        let (tx, rx) = channel();
        self.subscribers.push(tx);
        rx
    }

    fn notify(&mut self, message: &str) {
        // Remove disconnected subscribers
        self.subscribers.retain(|tx| tx.send(message.to_string()).is_ok());
    }
}

fn main() {
    println!("=== Approach 1: Traditional Observer ===");
    let mut subject = Subject::new();

    let observer1: Arc<dyn Observer> = Arc::new(ConcreteObserver::new("Observer 1"));
    let observer2: Arc<dyn Observer> = Arc::new(ConcreteObserver::new("Observer 2"));

    subject.subscribe(Arc::clone(&observer1));
    subject.subscribe(Arc::clone(&observer2));

    subject.notify("Hello, observers!");

    // Can unsubscribe
    subject.unsubscribe(&observer1);
    subject.notify("Observer 1 is gone");

    println!("\n=== Approach 2: Channel-based (Idiomatic) ===");
    let mut channel_subject = ChannelSubject::new();

    let rx1 = channel_subject.subscribe();
    let rx2 = channel_subject.subscribe();

    channel_subject.notify("Message 1");

    // Receivers can be moved to threads
    println!("Receiver 1: {}", rx1.recv().unwrap());
    println!("Receiver 2: {}", rx2.recv().unwrap());

    // Drop rx1 to unsubscribe
    drop(rx1);

    channel_subject.notify("Message 2");
    println!("Receiver 2: {}", rx2.recv().unwrap());
}