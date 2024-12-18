### Concurrency in Rust

Rust provides several ways to handle concurrency, ensuring safety and performance. Here are some key concepts and examples:

### 1. Threads

Rust's standard library provides a way to spawn threads using the `std::thread` module. The `thread::spawn` function is used to create a new thread. The || syntax is used to define a [closure](./closure.md), which is an anonymous function that can capture variables from its surrounding scope.

Here's a breakdown of the syntax:

`||`: This defines a closure with no parameters. If the closure had parameters, they would be listed between the vertical bars.
`{ ... }`: This is the body of the closure, which contains the code that will be executed in the new thread.

```rust
use std::thread;

macro_rules! sleep {
    ($millis:expr) => {
        std::thread::sleep(std::time::Duration::from_millis($millis));
    };
}

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            sleep!(50);
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        sleep!(50);
    }

    handle.join().unwrap();
}
```

### 2. Message Passing

Rust uses channels for message passing between threads, provided by the `std::sync::mpsc` module.

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```

### 3. Shared State

Rust ensures safe access to shared state using Arc (Atomic Reference Counting) and Mutex (Mutual Exclusion) from the `std::sync` module.

```rust
use std::sync::{Arc, Mutex};
use std::thread;


fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```
An atomic reference-counted pointer `Arc` is created to manage the shared ownership of a `Mutex` that guards an integer counter initialized to 0. <br/>

A vector `handles` is created to store the handles of the spawned threads.<br/>

A loop is used to spawn 10 threads. Each thread:<br/>
  - Clones the `Arc` pointer to get a new reference to the shared `Mutex`.<br/>
  - Locks the `Mutex` to get mutable access to the counter.
  - Increments the counter by 1.<br/>

Another loop is used to join all the spawned threads, ensuring that the main thread waits for all the threads to finish execution.<br/>

Finally, the value of the counter is printed, which should be 10 if all threads have successfully incremented the counter.<br/>
