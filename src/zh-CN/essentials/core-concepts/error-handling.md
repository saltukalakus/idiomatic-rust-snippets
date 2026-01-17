````markdown
### 错误处理

错误处理是编写健壮可靠软件的关键部分。Rust 提供了强大的错误处理工具，主要通过 `Result` 与 `Option` 类型来表达可能的错误或缺失值。

### `Result` 类型

`Result` 类型用于返回可能出错的函数。它是一个包含两个变体的枚举：`Ok(T)` 表示成功结果，`Err(E)` 表示错误。

```rust, editable
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    match divide(4.0, 2.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match divide(5.0, 0.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
```

### `Option` 类型

`Option` 用于表示可能存在或不存在的值。它是一个包含两个变体的枚举：`Some(T)` 表示有值，`None` 表示无值。

```rust, editable
fn find_word(s: &str, word: &str) -> Option<usize> {
    s.find(word)
}

fn main() {
    match find_word("hello world", "world") {
        Some(index) => println!("Found at index: {}", index),
        None => println!("Not found"),
    }
}
```

### `?` 操作符

`?` 操作符是传播错误的简写。它可用于返回 `Result` 或 `Option` 的函数中。

```rust, editable
use std::io;

fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = std::fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    match read_file("example.txt") {
        Ok(contents) => println!("File contents: {}", contents),
        Err(e) => println!("Error reading file: {}", e),
    }
}
```

### 自定义错误类型

可以自定义错误类型以提供更丰富的错误上下文。

```rust, editable
use std::fmt;

#[derive(Debug)]
enum MyError {
    IoError(std::io::Error),
    ParseError(std::num::ParseIntError),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MyError::IoError(ref err) => write!(f, "IO error: {}", err),
            MyError::ParseError(ref err) => write!(f, "Parse error: {}", err),
        }
    }
}

impl From<std::io::Error> for MyError {
    fn from(err: std::io::Error) -> MyError {
        MyError::IoError(err)
    }
}

impl From<std::num::ParseIntError> for MyError {
    fn from(err: std::num::ParseIntError) -> MyError {
        MyError::ParseError(err)
    }
}

fn read_and_parse_file(path: &str) -> Result<i32, MyError> {
    let contents = std::fs::read_to_string(path)?;
    let number: i32 = contents.trim().parse()?;
    Ok(number)
}

fn main() {
    match read_and_parse_file("number.txt") {
        Ok(number) => println!("Parsed number: {}", number),
        Err(e) => println!("Error: {}", e),
    }
}
```

我们定义了一个自定义错误类型 `MyError`，它可以表示 I/O 错误和解析错误。通过实现 `fmt::Display` 提供更友好的错误信息，并实现 `From` trait 以便从标准错误类型转换为 `MyError`，从而在 `read_and_parse_file` 中使用 `?` 操作符传播错误。

这些模式涵盖了 Rust 错误处理的基础。通过使用 `Result`、`Option`、`?` 操作符和自定义错误类型，可以编写能够优雅处理错误并提供有用反馈的代码。

````
