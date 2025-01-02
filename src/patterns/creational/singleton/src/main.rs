use std::sync::{Arc, Mutex};
use std::thread;

struct Singleton {
    data: i32,
}

impl Singleton {
    fn new() -> Self {
        Singleton { data: 0 }
    }

    fn get_instance() -> Arc<Mutex<Singleton>> {
        static mut SINGLETON: Option<Arc<Mutex<Singleton>>> = None;
        static ONCE: std::sync::Once = std::sync::Once::new();

        unsafe {
            ONCE.call_once(|| {
                let singleton = Singleton::new();
                SINGLETON = Some(Arc::new(Mutex::new(singleton)));
            });

            SINGLETON.clone().unwrap()
        }
    }

    fn increment(&mut self) {
        self.data += 1;
    }

    fn get_data(&self) -> i32 {
        self.data
    }
}

fn main() {
    let singleton = Singleton::get_instance();

    // Acquire the lock in the main thread
    {
        let mut instance = singleton.lock().unwrap();
        instance.increment();
        println!("Main thread data: {}", instance.get_data());
    } // The lock is released here when `instance` goes out of scope

    let singleton_clone1 = Arc::clone(&singleton);
    let singleton_clone2 = Arc::clone(&singleton);

    let handle1 = thread::spawn(move || {
        let mut instance1 = singleton_clone1.lock().unwrap();
        instance1.increment();
        println!("Thread 1 data: {}", instance1.get_data());
    });

    let handle2 = thread::spawn(move || {
        let mut instance2 = singleton_clone2.lock().unwrap();
        instance2.increment();
        println!("Thread 2 data: {}", instance2.get_data());
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    // Acquire the lock again in the main thread
    {
        let instance = singleton.lock().unwrap();
        println!("Final data: {}", instance.get_data());
    }
}