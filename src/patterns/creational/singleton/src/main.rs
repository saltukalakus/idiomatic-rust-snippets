use std::sync::{Arc, Mutex};
use std::thread;

struct Singleton {
    value: i32,
}

impl Singleton {
    fn new() -> Arc<Mutex<Singleton>> {
        Arc::new(Mutex::new(Singleton { value: 0 }))
    }

    fn get_instance() -> Arc<Mutex<Singleton>> {
        static mut SINGLETON: Option<Arc<Mutex<Singleton>>> = None;
        static ONCE: std::sync::Once = std::sync::Once::new();

        unsafe {
            ONCE.call_once(|| {
                SINGLETON = Some(Singleton::new());
            });
            SINGLETON.clone().unwrap()
        }
    }

    fn set_value(&mut self, value: i32) {
        self.value = value;
    }

    fn get_value(&self) -> i32 {
        self.value
    }
}

fn main() {
    let singleton = Singleton::get_instance();
    let singleton_clone = Arc::clone(&singleton);

    let handle = thread::spawn(move || {
        let mut instance = singleton_clone.lock().unwrap();
        instance.set_value(42);
    });

    handle.join().unwrap();

    let instance = singleton.lock().unwrap();
    println!("Singleton value: {}", instance.get_value());
}