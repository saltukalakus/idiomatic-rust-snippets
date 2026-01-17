### 类型状态模式

类型状态模式将对象的状态编码在其类型中，使无效的状态转换成为编译时错误。这利用 Rust 的类型系统为对象状态提供编译时保证。

**优势**：
- 无效的状态转换是编译时错误，而非运行时错误
- 自文档化的 API - 类型显示有效的操作
- 零运行时成本 - 状态是类型，而非值
- 无需运行时状态检查

```rust, editable
{{#include typestate/src/main.rs}}
```

**关键点**：
- 示例展示了 `Document<Draft>`、`Document<Review>` 和 `Document<Published>` 作为独立类型
- `Document<Draft>` 有 `write()` 和 `submit_for_review()` 方法；其他类型没有
- `submit_for_review()` 消费 `Document<Draft>` 并返回 `Document<Review>` - 类型级别的状态转换
- 在 `Document<Published>` 上调用 `doc.write()` 是编译错误 - 该类型不存在此方法
- 第二个示例：`ConnectionBuilder` 的 `build()` 仅在同时设置了主机和端口时可用
- `PhantomData<State>` 标记类型参数而不存储它 - 零运行时大小

**何时使用**：
- 状态机（文档、连接、协议）
- 具有必需字段的构建器
- 资源生命周期管理（打开/关闭、锁定/解锁）
- 任何具有严格状态转换的工作流
