### 消息传递（Message Passing）

消息传递允许发布者通过通道向订阅者通知事件。这将发布者与订阅者解耦，使代码更加模块化和线程安全。

**优势**：
- 避免共享可变状态 - 数据通过通道发送
- 自然的线程安全 - 通道可跨线程工作
- 清晰的所有权 - 订阅者拥有其接收端
- 当接收者被丢弃时自动清理

```rust, editable
{{#include ../../../patterns/behavioral/message-passing/src/main.rs}}
```

**关键点**：
- 示例展示了两种方法：使用 `Arc<dyn Observer>` 的传统观察者和基于通道的发布-订阅
- 传统方式：`Subject` 维护 `Vec<Arc<dyn Observer>>`，在 `notify()` 期间对每个调用 `update()`
- 基于通道：`ChannelSubject` 存储 `Vec<Sender<String>>`，通过 `send()` 发送消息
- 当订阅者超出作用域时，接收者自动丢弃 - 无需手动清理
- 通道方法避免了 trait 对象并提供更好的线程安全性

**何时使用**：
- 发布-订阅模式（事件总线、通知）
- 跨线程通信
- 解耦对状态变化做出反应的组件
- 当需要在订阅者丢弃时自动清理时