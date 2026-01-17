### RAII（资源获取即初始化）

RAII 是 Rust 中的基本模式，其中资源（内存、文件、锁、连接）在其所有者超出作用域时自动清理。这通过 Rust 的所有权系统和 `Drop` trait 实现。

**优势**：
- 无资源泄漏 - 清理有保证
- 异常安全 - 即使发生 panic 也会清理
- 无需垃圾回收器
- 显式的资源生命周期与作用域绑定

```rust, editable
{{#include raii/src/main.rs}}
```

**关键点**：
- 示例展示了在 `new()` 中创建的 `DatabaseConnection`（打印"Opening"），通过 `Drop` 自动关闭（打印"Closing"）
- 当 `conn` 超出作用域（右花括号）时，自动调用 `drop()`
- 嵌套示例：由于 LIFO 顺序（先内部作用域），`conn2` 在 `conn1` 之前 drop
- `might_fail()` 提前返回错误，但连接仍会关闭 - 即使提前返回，`Drop` 也会运行
- 无需手动清理 - Rust 的所有权确保 `Drop::drop()` 始终运行

**std 中的常见 RAII 类型**：
- `File` - drop 时关闭文件句柄
- `MutexGuard` - drop 时释放锁
- `Box`、`Vec`、`String` - drop 时释放内存
- `Rc`、`Arc` - drop 时减少引用计数
- `JoinHandle` - 可用于确保线程清理

**何时使用**：
- 任何需要清理的资源
- 文件句柄、网络连接、锁
- 必须恢复的临时状态
- 任何获取/释放模式
