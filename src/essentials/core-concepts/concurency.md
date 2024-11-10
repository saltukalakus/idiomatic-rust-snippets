### Concurrency in Rust

Rust provides several ways to handle concurrency, ensuring safety and performance. Here are some key concepts and examples:

### 1. Threads

Rust's standard library provides a way to spawn threads using the `std::thread` module. The `thread::spawn` function is used to create a new thread. The || syntax is used to define a [closure](./closures.md), which is an anonymous function that can capture variables from its surrounding scope.

Here's a breakdown of the syntax:

`||`: This defines a closure with no parameters. If the closure had parameters, they would be listed between the vertical bars.
`{ ... }`: This is the body of the closure, which contains the code that will be executed in the new thread.

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
    }

    handle.join().unwrap();
}
```

### 2. Message Passing

Rust uses channels for message passing between threads, provided by the `std::sync::mpsc` module.

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

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

Rust ensures safe access to shared state using `Mutex` and `Arc` from the `std::sync` module.

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

These examples demonstrate how Rust's ownership and type system ensure thread safety and prevent data races.
