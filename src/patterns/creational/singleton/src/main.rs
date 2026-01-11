use std::sync::{Arc, Mutex, OnceLock};
use std::thread;

// Global state using OnceLock - idiomatic and safe
struct AppConfig {
    data: i32,
}

impl AppConfig {
    fn new() -> Self {
        AppConfig { data: 0 }
    }

    fn increment(&mut self) {
        self.data += 1;
    }

    fn get_data(&self) -> i32 {
        self.data
    }
}

// Safe singleton using OnceLock - no unsafe code needed
fn get_config() -> &'static Arc<Mutex<AppConfig>> {
    static CONFIG: OnceLock<Arc<Mutex<AppConfig>>> = OnceLock::new();
    CONFIG.get_or_init(|| Arc::new(Mutex::new(AppConfig::new())))
}

fn main() {
    // Get the singleton instance
    let config = get_config();

    // Use in main thread
    {
        let mut instance = config.lock().unwrap();
        instance.increment();
        println!("Main thread data: {}", instance.get_data());
    }

    // Clone Arc for threads - all refer to same singleton
    let config1 = Arc::clone(config);
    let config2 = Arc::clone(config);

    let handle1 = thread::spawn(move || {
        let mut instance = config1.lock().unwrap();
        instance.increment();
        println!("Thread 1 data: {}", instance.get_data());
    });

    let handle2 = thread::spawn(move || {
        let mut instance = config2.lock().unwrap();
        instance.increment();
        println!("Thread 2 data: {}", instance.get_data());
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    // Verify final state
    {
        let instance = config.lock().unwrap();
        println!("Final data: {}", instance.get_data());
    }
}