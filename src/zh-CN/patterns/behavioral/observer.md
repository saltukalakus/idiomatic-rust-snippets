### 观察者（Observer）模式

观察者模式允许对象注册为主题（subject）的监听器，当主题状态变化时通知所有观察者。Rust 中可使用回调、channel 或 trait 对象实现。

```rust, editable
struct Subject { observers: Vec<Box<dyn Fn(i32)>> }

impl Subject {
    fn new() -> Self { Self { observers: vec![] } }
    fn register<F: 'static + Fn(i32)>(&mut self, f: F) { self.observers.push(Box::new(f)); }
    fn notify(&self, val: i32) { for o in &self.observers { o(val); } }
}
```
