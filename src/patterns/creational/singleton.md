### Singleton Design Pattern

The Singleton pattern ensures that a class has only one instance and provides a global point of access to it. This is useful when exactly one object is needed to coordinate actions across the system.

Here is an example of the Singleton Design Pattern:

```rust
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
```

1. **Singleton Struct**: Defines the structure of the singleton with a single field `value`.
2. **new() Method**: Creates a new instance of the singleton wrapped in `Arc` and `Mutex` for thread safety.
3. **get_instance() Method**: Provides a global point of access to the singleton instance. It uses `std::sync::Once` to ensure that the instance is only created once.
4. **set_value() and get_value() Methods**: Allow modification and retrieval of the singleton's value.
5. **main() Function**: Demonstrates the usage of the singleton in a multi-threaded context.

This example ensures that only one instance of `Singleton` is created and shared across threads safely.
