### Arc 与 Mutex

`Arc<T>` (原子引用计数) 和 `Mutex<T>` (互斥锁) 是 Rust 中用于安全并发编程的基本类型。它们支持跨线程的共享所有权和同步可变访问。

### Arc - 原子引用计数

`Arc<T>` 是一个线程安全的引用计数指针。多个线程可以拥有相同的数据，当最后一个 `Arc` 被丢弃时，数据将被释放。

```rust, editable
use std::sync::Arc;
use std::thread;

fn main() {
    let data = Arc::new(vec![1, 2, 3, 4, 5]);
    let mut handles = vec![];
    
    for i in 0..3 {
        let data_clone = Arc::clone(&data);
        
        let handle = thread::spawn(move || {
            println!("线程 {} 看到: {:?}", i, data_clone);
        });
        
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}
```

### Mutex - 互斥锁

`Mutex<T>` 提供具有线程安全锁定的内部可变性。一次只有一个线程可以访问数据。

```rust, editable
use std::sync::Mutex;

fn main() {
    let counter = Mutex::new(0);
    
    {
        let mut num = counter.lock().unwrap();
        *num += 1;
    } // 锁在这里被释放
    
    println!("计数器: {}", *counter.lock().unwrap());
}
```

### Arc<Mutex<T>> - 强大的组合

结合 `Arc` 和 `Mutex` 可以实现跨线程的共享可变状态：

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
    
    println!("最终计数: {}", *counter.lock().unwrap());
}
```

### Arc vs Rc

`Rc<T>` 是单线程的，而 `Arc<T>` 是线程安全的，使用原子操作：

```rust, editable
use std::rc::Rc;
use std::sync::Arc;
use std::thread;

fn main() {
    // 单线程引用计数
    let rc_data = Rc::new(5);
    let rc_clone = Rc::clone(&rc_data);
    
    // 线程安全的引用计数 (原子操作)
    let arc_data = Arc::new(5);
    let arc_clone = Arc::clone(&arc_data);
    
    // Rc 不能在线程间发送
    // thread::spawn(move || {
    //     println!("{}", rc_data); // 错误: Rc 不能在线程间发送
    // });
    
    // Arc 可以在线程间发送
    thread::spawn(move || {
        println!("{}", arc_data); // 正确
    }).join().unwrap();
}
```

### 锁守卫和 RAII

`Mutex::lock()` 返回一个 `MutexGuard`，它在被丢弃时自动释放锁：

```rust, editable
use std::sync::Mutex;

fn main() {
    let data = Mutex::new(vec![1, 2, 3]);
    
    {
        let mut guard = data.lock().unwrap();
        guard.push(4);
        // 当 guard 离开作用域时，锁被自动释放
    }
    
    println!("{:?}", data.lock().unwrap());
}
```

### 处理锁中毒

如果一个线程在持有锁时发生 panic，`Mutex` 就会“中毒”：

```rust, editable
use std::sync::Mutex;

fn main() {
    let mutex = Mutex::new(0);
    
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut guard = mutex.lock().unwrap();
        *guard += 1;
        panic!("线程 panic 了!");
    }));
    
    // Mutex 现在已中毒
    match mutex.lock() {
        Ok(guard) => println!("值: {}", *guard),
        Err(poisoned) => {
            println!("Mutex 已中毒!");
            // 仍然可以访问数据
            let guard = poisoned.into_inner();
            println!("恢复的值: {}", *guard);
        }
    };
}
```

### try_lock - 非阻塞尝试锁定

演示如何使用 `try_lock()` 尝试获取锁而不阻塞——如果锁已被持有，它会立即返回一个错误：

```rust, editable
use std::sync::Mutex;

fn main() {
    let mutex = Mutex::new(0);
    
    let guard = mutex.lock().unwrap();
    
    // 尝试非阻塞地获取锁
    match mutex.try_lock() {
        Ok(_) => println!("锁已获取"),
        Err(_) => println!("锁已被持有"),
    };
    
    drop(guard); // 释放锁
    
    match mutex.try_lock() {
        Ok(_) => println!("成功获取锁!"),
        Err(_) => println!("仍然无法获取锁"),
    };
}
```

### 死锁预防

在使用多个互斥锁时要小心避免死锁：

```rust, editable
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let resource1 = Arc::new(Mutex::new(0));
    let resource2 = Arc::new(Mutex::new(0));
    
    // 好的做法: 总是以相同的顺序获取锁
    let r1 = Arc::clone(&resource1);
    let r2 = Arc::clone(&resource2);
    let handle1 = thread::spawn(move || {
        let _guard1 = r1.lock().unwrap();
        let _guard2 = r2.lock().unwrap();
        println!("线程 1 获取了两个锁");
    });
    
    let r1 = Arc::clone(&resource1);
    let r2 = Arc::clone(&resource2);
    let handle2 = thread::spawn(move || {
        let _guard1 = r1.lock().unwrap(); // 相同顺序
        let _guard2 = r2.lock().unwrap();
        println!("线程 2 获取了两个锁");
    });
    
    handle1.join().unwrap();
    handle2.join().unwrap();
}
```

### RwLock - 读写锁

`RwLock` 允许多个读取者或一个写入者：

```rust, editable
use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));
    let mut handles = vec![];
    
    // 多个读取者可以同时访问
    for i in 0..5 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let reader = data_clone.read().unwrap();
            println!("读取者 {} 看到: {:?}", i, *reader);
        });
        handles.push(handle);
    }
    
    // 单个写入者拥有独占访问权
    let data_clone = Arc::clone(&data);
    let handle = thread::spawn(move || {
        let mut writer = data_clone.write().unwrap();
        writer.push(4);
        println!("写入者修改了数据");
    });
    handles.push(handle);
    
    for handle in handles {
        handle.join().unwrap();
    }
}
```

### 性能考虑

这个例子演示了 Arc 和 Mutex 的开销特性，而没有实际运行并发代码：

```rust,ignore
use std::sync::{Arc, Mutex};

fn main() {
    // Arc 具有原子操作开销
    let arc_data = Arc::new(5);
    
    // 克隆 Arc 很廉价 (只增加原子计数器)
    let clone1 = Arc::clone(&arc_data);
    let clone2 = Arc::clone(&arc_data);
    
    // Mutex 增加了同步开销
    let mutex_data = Mutex::new(vec![1, 2, 3]);
    
    // 加锁/解锁有成本 - 最小化临界区
    {
        let mut data = mutex_data.lock().unwrap();
        data.push(4);
    } // 尽快释放锁
}
```

### 常见模式

**共享配置**:

使用 Arc 在线程间共享不可变数据，而无需克隆底层数据：

```rust, editable
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
    
    // 在线程间共享配置 (不可变)
    let config_clone = Arc::clone(&config);
    std::thread::spawn(move || {
        println!("最大连接数: {}", config_clone.max_connections);
    }).join().unwrap();
}
```

**线程安全计数器**:

结合 Arc 和 Mutex 创建一个线程安全的计数器，可以从多个线程安全地递增：

```rust, editable
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
    
    println!("最终计数: {}", counter.get());
}
```

### 最佳实践

- **跨线程共享所有权时使用 `Arc`** - 单线程代码应使用 `Rc`
- **最小化临界区** - 尽可能短地持有锁
- **对于读多写少的负载，优先使用 `RwLock`** - 允许多个并发读取者
- **始终以一致的顺序获取锁** - 防止死锁
- **考虑无锁替代方案** - 对于更简单的情况，使用原子类型、通道
- **使用 `try_lock()` 避免阻塞** - 在适当的时候
- **处理中毒的互斥锁** - 决定是恢复还是传播错误
- **明确克隆 `Arc`** - 为清晰起见，使用 `Arc::clone(&arc)`，而不是 `arc.clone()`