### 中介者（Mediator）模式

中介者负责协调对象之间的交互，避免对象直接引用彼此，从而降低耦合。Rust 中可用 trait 与集中调度器实现类似功能。

```rust, editable
trait Mediator { fn notify(&self, sender: &str, event: &str); }

struct SimpleMediator;
impl Mediator for SimpleMediator {
    fn notify(&self, sender: &str, event: &str) { println!("{} triggered {}", sender, event); }
}
```
