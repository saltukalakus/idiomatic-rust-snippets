### Arc 与 Mutex

`Arc<T>`（原子引用计数）与 `Mutex<T>`（互斥锁）是 Rust 并发编程中用以保证线程安全的关键类型。它们允许跨线程共享所有权并提供同步的可变访问。

### Arc - 原子引用计数

`Arc<T>` 是线程安全的引用计数指针，允许多个线程拥有相同的数据；当最后一个 `Arc` 被释放时，数据被回收。

```rust, editable
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

### Mutex - 互斥锁

`Mutex<T>` 提供线程安全的内部可变性：同一时间只有一个线程可以访问数据。

```rust, editable
use std::sync::Mutex;

fn main() {
    let counter = Mutex::new(0);
    
    {
        let mut num = counter.lock().unwrap();
        *num += 1;
    } // 锁在此处释放
    
    println!("Counter: {}", *counter.lock().unwrap());
}
```

### Arc<Mutex<T>> - 强组合

将 `Arc` 与 `Mutex` 组合可以实现跨线程的共享可变状态：

```rust, editable
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

### Arc 与 Rc 的区别

`Rc<T>` 用于单线程，`Arc<T>` 在内部使用原子操作以保证线程安全：

```rust, editable
use std::rc::Rc;
use std::sync::Arc;
use std::thread;

fn main() {
    // 单线程引用计数
    let rc_data = Rc::new(5);
    let rc_clone = Rc::clone(&rc_data);
    
    // 线程安全的引用计数（原子操作）
    let arc_data = Arc::new(5);
    let arc_clone = Arc::clone(&arc_data);
    
    // Rc 无法在线程间发送
    // thread::spawn(move || {
    //     println!("{}", rc_data); // Error: Rc cannot be sent between threads
    // });
    
    // Arc 可以在线程间发送
    thread::spawn(move || {
        println!("{}", arc_data); // OK
    }).join().unwrap();
}
```

### 锁守护（Lock Guards）与 RAII

`Mutex::lock()` 返回一个 `MutexGuard`，当它被 drop 时会自动释放锁：

```rust, editable
use std::sync::Mutex;

fn main() {
    let data = Mutex::new(vec![1, 2, 3]);
    
    {
        let mut guard = data.lock().unwrap();
        guard.push(4);
        // 当 guard 超出作用域时，锁自动释放
    }
    
    println!("{:?}", data.lock().unwrap());
}
```

### 处理锁污染（Poisoning）

当持有锁的线程 panic 时，`Mutex` 会被标记为“poisoned”——可以选择恢复或传播错误：

```rust, editable
use std::sync::Mutex;

fn main() {
    let mutex = Mutex::new(0);
    
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut guard = mutex.lock().unwrap();
        *guard += 1;
        panic!("Thread panicked!");
    }));
    
    // Mutex 可能被标记为 poisoned
    match mutex.lock() {
        Ok(guard) => println!("Value: {}", *guard),
        Err(poisoned) => {
            println!("Mutex was poisoned!");
            // 仍然可以访问数据
            let guard = poisoned.into_inner();
            println!("Recovered value: {}", *guard);
        }
    };
}
```

### try_lock - 非阻塞获取锁

示例演示 `try_lock()` 在锁已被持有时立即返回错误：

```rust, editable
use std::sync::Mutex;

fn main() {
    let mutex = Mutex::new(0);
    
    let guard = mutex.lock().unwrap();
    
    // 尝试非阻塞获取锁
    match mutex.try_lock() {
        Ok(_) => println!("Lock acquired"),
        Err(_) => println!("Lock is already held"),
    };
    
    drop(guard); // 释放锁
    
    match mutex.try_lock() {
        Ok(_) => println!("Lock acquired successfully!"),
        Err(_) => println!("Still couldn't acquire lock"),
    };
}
```

I'll stop here due to length. Continue the rest? (I will create file)