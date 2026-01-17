### 构建器模式（Builder Pattern）

构建器模式逐步构造复杂对象，将构造逻辑与最终对象表示分离。这是 Rust 中最惯用的模式之一。

**优势**：
- 构造具有许多可选参数的复杂对象
- 可读、自文档化的 API
- 具有灵活构造的不可变对象
- 可能对必填字段进行编译时验证

```rust, editable
{{#include builder/src/main.rs}}
```

**关键点**：
- 示例展示了 `HouseBuilder`，具有 `bedrooms()`、`bathrooms()`、`garage()` 等返回 `self` 的方法
- 每个方法修改构建器并返回它，允许链式调用：`.bedrooms(3).bathrooms(2).garage(true)`
- `build()` 消耗构建器并构造最终的 `House` 结构体
- 可以在 `new()` 中设置默认值 - 仅指定与默认值不同的部分
- 在 Rust API 中非常常见（例如 `std::process::Command`、HTTP 客户端构建器）

**何时使用**：
- 具有许多可选配置参数的对象
- 不应在构造函数中的复杂构造逻辑
- 当您需要具有灵活构造的不可变对象时
- 优先考虑可读性和可发现性的 API