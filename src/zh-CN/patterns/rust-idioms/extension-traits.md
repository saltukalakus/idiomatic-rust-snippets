### 扩展

扩展 允许你为不属于你的类型添加方法。这是 Rust 中常见的模式，用于在不修改外部类型（如来自 `std` 或其他 crate 的类型）的情况下扩展其功能。

**优势**：
- 为来自外部 crate 的类型添加方法
- 保持实现的组织性和模块化
- 解决 Rust 的 trait 孤儿规则
- 在现有类型上创建特定领域的 API

```rust, editable
{{#include extension-traits/src/main.rs}}
```

**关键点**：
- 示例定义了带有 `is_blank()` 和 `truncate_with_ellipsis()` 方法的 `StringExt` trait
- 在 `str` 类型（不属于我们）上实现 - 为所有字符串切片添加如 `.is_blank()` 的方法
- `IteratorExt` 为所有迭代器添加 `sum_by()` 方法 - 在求和前映射元素
- `OptionExt` 添加 `ok_or_else_log()` 方法，在返回 `Err` 前将错误记录到 stderr
- `VecExt` 添加 `push_if_not_exists()` 方法，仅在值不在向量中时才推入
- 用户必须 `use` trait 才能访问扩展方法

**Rust 生态系统中的常见扩展 Trait**：
- `itertools::Itertools` - 用许多方法扩展 Iterator
- `futures::StreamExt` - async stream 扩展
- `anyhow::Context` - 用上下文方法扩展 Result

**何时使用**：
- 为标准库类型添加实用方法
- 为现有类型创建特定领域的 API
- 在库中提供便捷方法
- 当无法修改原始类型时
