### 糟糕的模式匹配

Rust 中的模式匹配本质上是穷尽的，但过早使用通配模式或忽略特定情况可能会导致错误。在应处理特定情况时使用 `_` 或通配匹配会导致逻辑错误和遗漏边缘情况。

穷尽的模式匹配是 Rust 的强项之一——编译器确保所有情况都得到处理。过于宽泛的模式会绕过此安全功能。

```rust, editable
enum Status {
    Success(String),
    Warning(String),
    Error(String),
}

fn process_status(status: Status) -> String {
    match status {
        Status::Success(msg) => format!("OK: {}", msg),
        _ => "发生了某事".to_string(), // 过于宽泛 - 丢失信息
    }
}

fn main() {
    let warning = Status::Warning("内存不足".to_string());
    let error = Status::Error("连接失败".to_string());
    
    println!("{}", process_status(warning)); // 打印: 发生了某事
    println!("{}", process_status(error));   // 打印: 发生了某事
}
```

通配模式 `_` 吞噬了重要信息。警告和错误产生相同的输出，使得调试变得不可能。如果稍后向枚举中添加新变体，此代码不会编译失败——它将静默地错误处理新情况。程序可以运行，但对于关键状态会产生无用的输出。

下一个示例使用穷尽的模式匹配。

```rust, editable
enum Status {
    Success(String),
    Warning(String),
    Error(String),
}

fn process_status(status: Status) -> String {
    match status {
        Status::Success(msg) => format!("✓ 成功: {}", msg),
        Status::Warning(msg) => format!("⚠ 警告: {}", msg),
        Status::Error(msg) => format!("✗ 错误: {}", msg),
    }
}

fn main() {
    let warning = Status::Warning("内存不足".to_string());
    let error = Status::Error("连接失败".to_string());
    
    println!("{}", process_status(warning)); // 打印: ⚠ 警告: 内存不足
    println!("{}", process_status(error));   // 打印: ✗ 错误: 连接失败
}
```

**关键改进**:
- 每个枚举变体都通过适当的逻辑进行显式处理
- 输出保留了原始状态的所有信息
- 如果添加了新变体，代码在处理它之前不会编译
- 编译器强制执行穷尽性，防止静默失败
- 每种状态类型的清晰、独特的输出有助于调试