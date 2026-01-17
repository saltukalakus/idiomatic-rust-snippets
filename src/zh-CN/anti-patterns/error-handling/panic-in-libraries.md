````markdown
### 库中的 Panic

库不应在正常操作中发生 panic —— 应通过返回 `Result` 类型让调用方决定如何处理错误。当库发生 panic 时，会将该 panic 强加给所有使用该库的应用。应用程序应该决定如何处理错误，而不是库。panic 违反了最小惊讶原则。

一个会 panic 的库相当于在说“我比你更懂如何处理这个错误”——这种情况几乎总是不成立。

```rust, editable
// Library code - BAD
pub fn parse_port(s: &str) -> u16 {
    if s.is_empty() {
        panic!("Port string cannot be empty");
    }
    
    match s.parse::<u16>() {
        Ok(port) if port > 0 => port,
        Ok(_) => panic!("Port must be greater than 0"),
        Err(_) => panic!("Invalid port number: {}", s),
    }
}

// Application code
fn main() {
    let ports = vec!["8080", "0", "invalid", "3000"];
    
    for port_str in ports {
        let port = parse_port(port_str); // Panics on "0" - crashes the entire app
        println!("Using port: {}", port);
    }
}
```

该库在输入无效时会 panic（例如 "0" 或 "invalid"），导致整个应用崩溃。应用无法恢复、记录错误、使用默认值或跳过无效端口。库代码剥夺了应用开发者的控制权。程序崩溃时会显示：`thread 'main' panicked at 'Port must be greater than 0'`。

下面的示例通过返回 `Result` 而不是 panic 来改进。

```rust, editable
// Library code - GOOD
pub fn parse_port(s: &str) -> Result<u16, String> {
    if s.is_empty() {
        return Err("Port string cannot be empty".to_string());
    }
    
    match s.parse::<u16>() {
        Ok(port) if port > 0 => Ok(port),
        Ok(_) => Err("Port must be greater than 0".to_string()),
        Err(_) => Err(format!("Invalid port number: {}", s)),
    }
}

// Application code
fn main() {
    let ports = vec!["8080", "0", "invalid", "3000"];
    
    for port_str in ports {
        match parse_port(port_str) {
            Ok(port) => println!("Using port: {}", port),
            Err(e) => eprintln!("Skipping invalid port '{}': {}", port_str, e),
        }
    }
    // Prints:
    // Using port: 8080
    // Skipping invalid port '0': Port must be greater than 0
    // Skipping invalid port 'invalid': Invalid port number: invalid
    // Using port: 3000
}
```

**关键改进**：
- 返回 `Result<u16, String>` —— 错误作为值而非崩溃。
- 即使存在无效输入，应用也能继续运行。
- 调用者可以决定如何处理错误 —— 跳过、使用默认值、重试或退出。
- 错误包含上下文，可用于记录或展示给用户。
- 库遵循 Rust 的惯例 —— 可失败的操作应返回 `Result`。

````