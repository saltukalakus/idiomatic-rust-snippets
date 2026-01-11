### Arc and Mutex

`Arc<T>` (Atomic Reference Counted) and `Mutex<T>` (Mutual Exclusion) are essential types for safe concurrent programming in Rust. They enable shared ownership and synchronized mutable access across threads.

### Arc - Atomic Reference Counting

`Arc<T>` is a thread-safe reference-counting pointer. Multiple threads can own the same data, and the data is deallocated when the last `Arc` is dropped.

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let data = Arc::new(vec![1, 2, 3, 4, 5]);
    let mut handles = vec![];
    
    for i in 0..3 {
        let data_clone = Arc::clone(&data);
        
        let handle = thread::spawn(move || {
            println!("Thread {} sees: {:?}", i, data_clone);
        });
        
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}
```

### Mutex - Mutual Exclusion

`Mutex<T>` provides interior mutability with thread-safe locking. Only one thread can access the data at a time.

```rust
use std::sync::Mutex;

fn main() {
    let counter = Mutex::new(0);
    
    {
        let mut num = counter.lock().unwrap();
        *num += 1;
    } // Lock is released here
    
    println!("Counter: {}", *counter.lock().unwrap());
}
```

### Arc<Mutex<T>> - The Power Combination

Combining `Arc` and `Mutex` enables shared mutable state across threads:

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        });
        
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final count: {}", *counter.lock().unwrap());
}
```

### Arc vs Rc

`Rc<T>` is single-threaded, `Arc<T>` is thread-safe with atomic operations:

```rust
use std::rc::Rc;
use std::sync::Arc;

fn main() {
    // Single-threaded reference counting
    let rc_data = Rc::new(5);
    let rc_clone = Rc::clone(&rc_data);
    
    // Thread-safe reference counting (atomic operations)
    let arc_data = Arc::new(5);
    let arc_clone = Arc::clone(&arc_data);
    
    // Rc cannot be sent between threads
    // thread::spawn(move || {
    //     println!("{}", rc_data); // Error: Rc cannot be sent between threads
    // });
    
    // Arc can be sent between threads
    thread::spawn(move || {
        println!("{}", arc_data); // OK
    }).join().unwrap();
}
```

### Lock Guards and RAII

`Mutex::lock()` returns a `MutexGuard` that automatically releases the lock when dropped:

```rust
use std::sync::Mutex;

fn main() {
    let data = Mutex::new(vec![1, 2, 3]);
    
    {
        let mut guard = data.lock().unwrap();
        guard.push(4);
        // Lock automatically released when guard goes out of scope
    }
    
    println!("{:?}", data.lock().unwrap());
}
```

### Handling Lock Poisoning

A `Mutex` becomes poisoned if a thread panics while holding the lock:

```rust
use std::sync::Mutex;

fn main() {
    let mutex = Mutex::new(0);
    
    let _ = std::panic::catch_unwind(|| {
        let mut guard = mutex.lock().unwrap();
        *guard += 1;
        panic!("Thread panicked!");
    });
    
    // Mutex is now poisoned
    match mutex.lock() {
        Ok(guard) => println!("Value: {}", *guard),
        Err(poisoned) => {
            println!("Mutex was poisoned!");
            // Can still access the data
            let guard = poisoned.into_inner();
            println!("Recovered value: {}", *guard);
        }
    }
}
```

### try_lock - Non-blocking Lock Attempt

```rust
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn main() {
    let mutex = Mutex::new(0);
    
    let guard = mutex.lock().unwrap();
    
    // Try to acquire lock without blocking
    match mutex.try_lock() {
        Ok(_) => println!("Lock acquired"),
        Err(_) => println!("Lock is already held"),
    }
    
    drop(guard); // Release the lock
    
    match mutex.try_lock() {
        Ok(_) => println!("Lock acquired successfully!"),
        Err(_) => println!("Still couldn't acquire lock"),
    }
}
```

### Deadlock Prevention

Be careful to avoid deadlocks when using multiple mutexes:

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let resource1 = Arc::new(Mutex::new(0));
    let resource2 = Arc::new(Mutex::new(0));
    
    // Good: Always acquire locks in the same order
    let r1 = Arc::clone(&resource1);
    let r2 = Arc::clone(&resource2);
    let handle1 = thread::spawn(move || {
        let _guard1 = r1.lock().unwrap();
        let _guard2 = r2.lock().unwrap();
        println!("Thread 1 acquired both locks");
    });
    
    let r1 = Arc::clone(&resource1);
    let r2 = Arc::clone(&resource2);
    let handle2 = thread::spawn(move || {
        let _guard1 = r1.lock().unwrap(); // Same order
        let _guard2 = r2.lock().unwrap();
        println!("Thread 2 acquired both locks");
    });
    
    handle1.join().unwrap();
    handle2.join().unwrap();
}
```

### RwLock - Read-Write Lock

`RwLock` allows multiple readers or one writer:

```rust
use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));
    let mut handles = vec![];
    
    // Multiple readers can access simultaneously
    for i in 0..5 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let reader = data_clone.read().unwrap();
            println!("Reader {} sees: {:?}", i, *reader);
        });
        handles.push(handle);
    }
    
    // Single writer has exclusive access
    let data_clone = Arc::clone(&data);
    let handle = thread::spawn(move || {
        let mut writer = data_clone.write().unwrap();
        writer.push(4);
        println!("Writer modified data");
    });
    handles.push(handle);
    
    for handle in handles {
        handle.join().unwrap();
    }
}
```

### Performance Considerations

```rust
use std::sync::{Arc, Mutex};

fn main() {
    // Arc has atomic overhead
    let arc_data = Arc::new(5);
    
    // Cloning Arc is cheap (just increments atomic counter)
    let clone1 = Arc::clone(&arc_data);
    let clone2 = Arc::clone(&arc_data);
    
    // Mutex adds synchronization overhead
    let mutex_data = Mutex::new(vec![1, 2, 3]);
    
    // Lock/unlock has cost - minimize critical sections
    {
        let mut data = mutex_data.lock().unwrap();
        data.push(4);
    } // Release lock as soon as possible
}
```

### Common Patterns

**Shared Configuration**:

```rust
use std::sync::Arc;

struct Config {
    max_connections: usize,
    timeout: u64,
}

fn main() {
    let config = Arc::new(Config {
        max_connections: 100,
        timeout: 30,
    });
    
    // Share config across threads (immutable)
    let config_clone = Arc::clone(&config);
    std::thread::spawn(move || {
        println!("Max connections: {}", config_clone.max_connections);
    }).join().unwrap();
}
```

**Thread-Safe Counter**:

```rust
use std::sync::{Arc, Mutex};
use std::thread;

struct Counter {
    count: Arc<Mutex<i32>>,
}

impl Counter {
    fn new() -> Self {
        Counter {
            count: Arc::new(Mutex::new(0)),
        }
    }
    
    fn increment(&self) {
        let mut count = self.count.lock().unwrap();
        *count += 1;
    }
    
    fn get(&self) -> i32 {
        *self.count.lock().unwrap()
    }
}

impl Clone for Counter {
    fn clone(&self) -> Self {
        Counter {
            count: Arc::clone(&self.count),
        }
    }
}

fn main() {
    let counter = Counter::new();
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter_clone = counter.clone();
        let handle = thread::spawn(move || {
            counter_clone.increment();
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final count: {}", counter.get());
}
```

### Best Practices

- **Use `Arc` for shared ownership across threads** - single-threaded code should use `Rc`
- **Minimize critical sections** - hold locks for as short a time as possible
- **Prefer `RwLock` for read-heavy workloads** - allows concurrent readers
- **Always acquire locks in consistent order** - prevents deadlocks
- **Consider lock-free alternatives** - atomics, channels for simpler cases
- **Use `try_lock()` to avoid blocking** - when appropriate
- **Handle poisoned mutexes** - decide whether to recover or propagate the error
- **Clone `Arc` explicitly** - use `Arc::clone(&arc)` for clarity, not `arc.clone()`
