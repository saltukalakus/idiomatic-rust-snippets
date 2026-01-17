### 构建器模式（方法链）

带有方法链的构建器模式允许通过链接方法调用来组合行为。每个方法都会修改并返回 `self`，从而实现流畅的 API 和零成本抽象。

**优点**:
- 零堆分配 - 完全基于栈
- 编译时分发 - 没有 vtable 开销
- 流畅、可读的 API
- 编译时类型安全的组合

```rust, editable
{{#include builder-pattern/src/main.rs}}
```

**关键点**:
- 该示例比较了 trait 对象方法（`Box<dyn Text>`）和方法链
- Trait 对象版本：`Bold` 包装 `Italic`，`Italic` 包装 `PlainText` - 每个都在堆上分配
- 方法链版本：`TextBuilder` 具有 `content`、`bold` 和 `italic` 字段
- 每个构建器方法（`bold()`、`italic()`）设置一个标志并返回 `self`
- `render()` 根据标志应用格式 - 全部在栈上分配，零堆开销

**何时使用**:
- 组合可选功能或行为
- 构建流畅的 API
- 运行时分发的零成本抽象
- 当行为集合在编译时已知时