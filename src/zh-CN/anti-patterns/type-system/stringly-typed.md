### 使用字符串表示固定集合

使用字符串表示具有固定取值范围的数据是脆弱且容易出错的。拼写错误、大小写敏感问题和无效值不能在编译期被捕获。基于字符串的类型绕过了 Rust 的穷尽匹配和类型安全。

枚举（enum）可以使无效状态不可表示。如果数据具有固定的取值集合，应使用枚举。

```rust, editable
fn process_status(status: &str) -> String {
    match status {
        "success" => "Operation completed".to_string(),
        "warning" => "Operation completed with warnings".to_string(),
        "error" => "Operation failed".to_string(),
        _ => "Unknown status".to_string(), // 在运行时捕获拼写错误
    }
}

fn set_log_level(level: &str) {
    match level {
        "debug" | "info" | "warn" | "error" => {
            println!("Log level set to: {}", level);
        }
        _ => println!("Invalid log level: {}", level), // 运行时错误
    }
}

fn main() {
    println!("{}", process_status("success"));
    println!("{}", process_status("sucess")); // 拼写错误 - 打印 "Unknown status"
    
    set_log_level("info");
    set_log_level("Info"); // 大小写敏感 - 打印 "Invalid log level"
    set_log_level("verbose"); // 无效值 - 打印 "Invalid log level"
}
```

字符串参数接受任意值 —— 拼写错误（如 "sucess"）或无效值（如 "verbose"）会在编译时通过，但会在运行时失败。大小写敏感会导致问题："Info" 与 "info" 不同。万用分支 `_` 隐藏问题而不是防止它们。添加新状态需要更新所有匹配语句，编译器不会帮你定位。

下面的示例使用枚举以提高类型安全。

```rust, editable
#[derive(Debug)]
enum Status {
    Success,
    Warning,
    Error,
}

#[derive(Debug)]
enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

fn process_status(status: Status) -> String {
    match status {
        Status::Success => "Operation completed".to_string(),
        Status::Warning => "Operation completed with warnings".to_string(),
        Status::Error => "Operation failed".to_string(),
    }
}

fn set_log_level(level: LogLevel) {
    println!("Log level set to: {:?}", level);
}

fn main() {
    println!("{}", process_status(Status::Success));
    // println!("{}", process_status(Status::Sucess)); // 编译错误 - 变体不存在
    
    set_log_level(LogLevel::Info);
    // set_log_level(LogLevel::Verbose); // 编译错误 - 变体不存在
    // 无大小写敏感问题 - Status::SUCCESS 并不存在
}
```

**关键改进**：
- 拼写错误成为编译错误而非运行时 bug
- 无法传入无效值 —— 仅存在定义的变体
- 无大小写敏感问题 —— 枚举变体具有确定名称
- 穷尽匹配 —— 编译器确保所有分支均被处理
- 增加新变体时，所有匹配语句必须更新（若未更新会导致编译错误）

