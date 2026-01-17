### 基于枚举的多态（Enum Polymorphism）

在 Rust 中，枚举可用于表达不同变体的多态行为，通常配合 `match` 或实现 trait 的方式进行分发。

```rust, editable
enum Message { Quit, Echo(String), Move{x:i32,y:i32} }

fn process(msg: Message) {
    match msg {
        Message::Quit => println!("退出"),
        Message::Echo(s) => println!("回显: {}", s),
        Message::Move{x,y} => println!("移动到 {},{}", x, y),
    }
}
```
