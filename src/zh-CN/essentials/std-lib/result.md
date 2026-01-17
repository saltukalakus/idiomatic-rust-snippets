### Result 类型

`Result` 类型是 Rust 处理可能失败操作的主要机制。它在编译时强制显式错误处理，防止错误在运行时被默默传播。

### 定义

`Result` 枚举在标准库中定义为：

```rust,ignore
enum Result<T, E> {
    Ok(T),    // 成功情况，包含类型为 T 的值
    Err(E),   // 错误情况，包含类型为 E 的错误
}
```

### 基本用法

```rust, editable
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn main() {
    match divide(10.0, 2.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### 使用 `?` 运算符处理 `Result`

`?` 运算符用于简洁地传播错误：如果 `Result` 是 `Err`，则函数会提前返回该错误；如果是 `Ok`，则解包出内部值：

```rust, editable
fn parse_number(s: &str) -> Result<i32, String> {
    s.parse::<i32>()
        .map_err(|e| format!("Failed to parse '{}': {}", s, e))
}

fn add_two_numbers(a: &str, b: &str) -> Result<i32, String> {
    let num_a = parse_number(a)?;  // 若解析失败则返回错误
    let num_b = parse_number(b)?;  // 若解析失败则返回错误
    Ok(num_a + num_b)
}

fn main() {
    // 成功示例
    match add_two_numbers("10", "20") {
        Ok(sum) => println!("Sum: {}", sum),
        Err(e) => println!("Error: {}", e),
    }
    
    // 错误示例
    match add_two_numbers("10", "abc") {
        Ok(sum) => println!("Sum: {}", sum),
        Err(e) => println!("Error: {}", e),
    }
}
```

### 常用 `Result` 方法

**检查变体**：
- `is_ok()`: 若为 `Ok` 返回 `true`
- `is_err()`: 若为 `Err` 返回 `true`

**提取值**：
- `unwrap()`: 返回 `Ok` 内部值或 panic（谨慎使用）
- `expect(msg)`: 与 `unwrap()` 类似，但可自定义 panic 信息
- `unwrap_or(default)`: 返回值或默认值
- `unwrap_or_else(f)`: 返回值或通过闭包计算默认值

```rust, editable
fn main() {
    let good_result: Result<i32, &str> = Ok(10);
    let bad_result: Result<i32, &str> = Err("error");
    
    // 检查
    println!("Is ok: {}", good_result.is_ok());  // true
    println!("Is err: {}", bad_result.is_err()); // true
    
    // 使用默认值提取
    println!("{}", good_result.unwrap_or(0));     // 10
    println!("{}", bad_result.unwrap_or(0));      // 0
    
    println!("{}", bad_result.unwrap_or_else(|e| {
        eprintln!("Error occurred: {}", e);
        -1
    })); // -1
}
```

### 转换 `Result`

**`map()` 与 `map_err()`**：转换 `Ok` 值或 `Err` 错误

```rust, editable
fn main() {
    let result: Result<i32, &str> = Ok(2);
    
    // 转换 Ok 值
    let doubled = result.map(|x| x * 2);
    println!("{:?}", doubled); // Ok(4)
    
    // 转换 Err 值
    let err: Result<i32, &str> = Err("oops");
    let mapped_err = err.map_err(|e| format!("Error: {}", e));
    println!("{:?}", mapped_err); // Err("Error: oops")
}
```

**`and_then()`**：链接返回 `Result` 的操作

```rust, editable
fn parse_number(s: &str) -> Result<i32, String> {
    s.parse().map_err(|_| format!("Not a number: {}", s))
}

fn double_if_positive(n: i32) -> Result<i32, String> {
    if n > 0 {
        Ok(n * 2)
    } else {
        Err("Number must be positive".to_string())
    }
}

fn main() {
    let result = parse_number("21")
        .and_then(double_if_positive);
    
    println!("{:?}", result); // Ok(42)
}
```

### 收集 `Result`

在处理 `Result` 的迭代器时，可以将它们收集成单个 `Result`：

```rust, editable
fn parse_numbers(strings: Vec<&str>) -> Result<Vec<i32>, std::num::ParseIntError> {
    strings.into_iter()
        .map(|s| s.parse::<i32>())
        .collect()  // 将 Vec<Result<i32, E>> 收集为 Result<Vec<i32>, E>
}

fn main() {
    match parse_numbers(vec!["1", "2", "3"]) {
        Ok(numbers) => println!("Numbers: {:?}", numbers),
        Err(e) => eprintln!("Parse error: {}", e),
    }
}
```

### 自定义错误类型

对于复杂应用，定义自定义错误类型：

```rust, editable
use std::fmt;

#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "Cannot divide by zero"),
            MathError::NegativeSquareRoot => write!(f, "Cannot take square root of negative"),
        }
    }
}

impl std::error::Error for MathError {}

fn divide(a: f64, b: f64) -> Result<f64, MathError> {
    if b == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

fn main() {
    match divide(10.0, 0.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
```

### Result 与 Option

- 当你需要提供错误信息时使用 `Result<T, E>`
- 当值缺失是预期且无需错误细节时使用 `Option<T>`

```rust,ignore
fn find_user(id: u32) -> Option<String> {
    // None just means "not found", no error details needed
    if id == 1 { Some("Alice".to_string()) } else { None }
}

fn load_config(path: &str) -> Result<String, std::io::Error> {
    // Err provides details about what went wrong
    std::fs::read_to_string(path)
}
```

### 在 `Result` 和 `Option` 之间转换

```rust, editable
fn main() {
    let result: Result<i32, &str> = Ok(5);
    let option: Option<i32> = result.ok();  // Ok(5) -> Some(5)
    
    let result2: Result<i32, &str> = option.ok_or("missing value");  // Some(5) -> Ok(5)
    
    println!("{:?}", result2);
}
```

### 最佳实践

- **对返回 `Result` 的函数使用 `?` 运算符** 来简洁传播错误
- **在生产代码中避免 `unwrap()`**——使用带描述性信息的 `expect()` 或适当的错误处理
- **库函数应返回 `Result`**，让调用者决定如何处理错误
- **对领域特定错误使用自定义错误类型**
- **利用 `map()`、`and_then()`** 进行转换与链接操作
- **对于复杂应用考虑使用 `anyhow` 或 `thiserror` 等错误处理库**
