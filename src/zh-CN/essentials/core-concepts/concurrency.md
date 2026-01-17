### 并发

并发是同时处理多个任务的机制，能更高效地利用系统资源（如 CPU）。当代码在不同线程的上下文中运行时，消息传递与共享变量的处理就变得非常重要。Rust 的标准库为这些并发需求提供了开箱即用的解决方案。

#### 线程

Rust 标准库通过 `std::thread` 模块提供了生成线程的方法。使用 `thread::spawn` 创建新线程。`||` 语法用于定义闭包（closure），它是可以从外层作用域捕获变量的匿名函数。

```rust, editable
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(50));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(50));
    }

    handle.join().unwrap();
}
```

- `||`：定义无参数的闭包。如果闭包有参数，它们会写在竖线之间。
- `{ ... }`：闭包体，包含将在新线程中执行的代码。
- `thread::sleep(Duration)`：使当前线程暂停指定时间。

#### 消息传递

```rust, editable
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
- 使用 `std::sync::mpsc` 模块提供的通道在线程间进行消息传递。

#### 共享状态

```rust, editable
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
- 使用原子引用计数指针 `Arc` 来管理对 `Mutex` 的共享所有权，`Mutex` 用于保护整数计数器的并发访问。
- 使用 `handles` 向量保存线程句柄以便随后 join。
- 每个线程克隆 `Arc`，锁住 `Mutex`，并将计数器加 1。
- 使用 join 等待所有线程完成，最后打印计数器值（应为 10）。

