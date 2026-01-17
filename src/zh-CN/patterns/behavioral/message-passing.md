### 消息传递（Message Passing）模式

消息传递是一种并发模型，通过发送消息在任务或线程之间通信。Rust 的 `mpsc` 通道与 `async`/`await` 生态使消息传递成为主要并发模式之一。

```rust, editable
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || tx.send(42).unwrap());
    println!("接收：{}", rx.recv().unwrap());
}
```
