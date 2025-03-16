### Concurrency

Concurrency is the mechanism of handling tasks in parallel. This helps use the available system resources such as CPUs more efficiently. As code runs in different contexts in threads, passing messages and handling shared variables becomes an important part of the implementation. Rust's standard library provides solutions to these concurrency needs out of the box.


#### Threads

Rust's standard library provides a way to spawn threads using the `std::thread` module. The `thread::spawn` function is used to create a new thread. The || syntax is used to define a [closure](./closure.md), which is an anonymous function that can capture variables from its surrounding scope.

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

- `||`: This defines a closure with no parameters. If the closure had parameters, they would be listed between the vertical bars.
- `{ ... }`: This is the body of the closure, which contains the code that will be executed in the new thread.

#### Message Passing

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
- channels from `std::sync::mpsc` module can be used for message passing between threads

#### Shared State

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
- An atomic reference-counted pointer `Arc` is created to manage the shared ownership of a `Mutex` that guards an integer counter initialized to 0. <br/>

- A vector `handles` is created to store the handles of the spawned threads.<br/>

- A loop is used to spawn 10 threads. Each thread:<br/>
  - Clones the `Arc` pointer to get a new reference to the shared `Mutex`.<br/>
  - Locks the `Mutex` to get mutable access to the counter.
  - Increments the counter by 1.<br/>

- Another loop is used to join all the spawned threads, ensuring that the main thread waits for all the threads to finish execution.<br/>

- Finally, the value of the counter is printed, which should be 10 if all threads have successfully incremented the counter.<br/>
