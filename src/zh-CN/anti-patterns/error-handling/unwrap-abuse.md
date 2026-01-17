### 滥用 unwrap

在 `Result` 和 `Option` 上调用 `.unwrap()` 或 `.expect()` 会将可恢复的错误变成 panic。示例、测试或原型中可以临时使用，但生产代码应优雅地处理错误。滥用 unwrap 会将错误处理强制为 panic，而不是显式的错误处理。

Rust 的类型系统通过 `Result<T, E>` 明确表示错误。使用 unwrap 会丢弃这种安全性，使 Rust 行为类似会崩溃的语言。

```rust, editable
use std::fs;
use std::env;

fn read_config() -> String {
    let path = env::var("CONFIG_PATH").unwrap(); // 若环境变量未设置则 panic
    let contents = fs::read_to_string(&path).unwrap(); // 若文件不存在则 panic
    contents
}

fn main() {
    let config = read_config();
    println!("配置已加载: {} 字节", config.len());
}
```

若 `CONFIG_PATH` 未设置或文件不存在，上述代码会 panic，panic 信息通常为：`thread 'main' panicked at 'called Option::unwrap() on a None value'`。没有关于失败原因的详细上下文，程序崩溃而不能优雅恢复，用户仅看到难以理解的 panic 信息。

下面的示例展示了正确的错误传播方式。

```rust, editable
use std::fs;
use std::env;
use std::error::Error;

fn read_config() -> Result<String, Box<dyn Error>> {
    let path = env::var("CONFIG_PATH")?;
    let contents = fs::read_to_string(&path)?;
    Ok(contents)
}

fn main() {
    match read_config() {
        Ok(config) => {
            println!("配置已加载: {} 字节", config.len());
        }
        Err(e) => {
            eprintln!("加载配置失败: {}", e);
            eprintln!("请设置 CONFIG_PATH 环境变量");
            std::process::exit(1);
        }
    }
}
```

**关键改进**：
- 返回 `Result` 而非 panic，使调用者可以处理错误。
- 使用 `?` 操作符实现简洁的错误传播。
- 提供友好的错误信息而非 panic 回溯。
- 可以优雅地退出（如 `exit(1)`）而不是 panic。
- 调用者可选择如何处理错误 —— 重试、使用默认值或退出。

