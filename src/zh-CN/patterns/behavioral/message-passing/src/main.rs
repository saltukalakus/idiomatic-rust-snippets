use std::sync::Arc;
use std::sync::mpsc::{channel, Sender};

// 方法 1：传统观察者模式，具有适当的所有权
// 使用弱引用允许取消订阅

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
    // 使用 Arc 进行共享所有权 - 可以移除观察者
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

// 方法 2：使用通道的惯用 Rust 方式（推荐）
// 这更灵活，并遵循 Rust 的消息传递哲学

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
        // 移除断开连接的订阅者
        self.subscribers.retain(|tx| tx.send(message.to_string()).is_ok());
    }
}

fn main() {
    println!("=== Approach 1: Traditional 观察者 ===");
    let mut subject = Subject::new();

    let observer1: Arc<dyn Observer> = Arc::new(ConcreteObserver::new("观察者 1"));
    let observer2: Arc<dyn Observer> = Arc::new(ConcreteObserver::new("观察者 2"));

    subject.subscribe(Arc::clone(&observer1));
    subject.subscribe(Arc::clone(&observer2));

    subject.notify("你好, observers!");

    // Can unsubscribe
    subject.unsubscribe(&observer1);
    subject.notify("观察者 1 is gone");

    println!("\n=== Approach 2: Channel-based (Idiomatic) ===");
    let mut channel_subject = ChannelSubject::new();

    let rx1 = channel_subject.subscribe();
    let rx2 = channel_subject.subscribe();

    channel_subject.notify("消息 1");

    // 接收者可以移动到线程
    println!("Receiver 1: {}", rx1.recv().unwrap());
    println!("Receiver 2: {}", rx2.recv().unwrap());

    // 丢弃 rx1 以取消订阅
    drop(rx1);

    channel_subject.notify("消息 2");
    println!("Receiver 2: {}", rx2.recv().unwrap());
}