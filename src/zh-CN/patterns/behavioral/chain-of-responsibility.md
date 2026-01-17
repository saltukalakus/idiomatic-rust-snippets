### 责任链模式

责任链模式将请求沿着处理程序链传递。每个处理程序决定是处理请求还是将其传递给下一个处理程序。

**优势**：
- 解耦请求发送者和接收者
- 动态配置处理程序链
- 每个处理程序具有单一职责
- 易于添加或重排处理程序

```rust, editable
{{#include chain-of-responsibility/src/main.rs}}
```

**关键点**：
- 示例定义了带有 `handle(&mut self, request: &str)` 方法的 `Handler` trait
- `BaseHandler` 存储 `Option<Box<dyn Handler>>` 作为链中的下一个处理程序
- `ConcreteHandlerA` 处理以 'A' 开头的请求，否则传递给下一个
- `ConcreteHandlerB` 处理以 'B' 开头的请求，否则传递给下一个
- 在 `main()` 中，构建链：处理程序 A → 处理程序 B，请求流经直到匹配

**何时使用**：
- 多个对象可能处理请求
- 处理程序集应动态确定
- 中间件管道（日志、认证、验证）
- 具有回退行为的事件处理系统