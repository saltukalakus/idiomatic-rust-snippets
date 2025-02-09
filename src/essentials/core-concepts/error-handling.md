### Error Handling in Rust

Error handling in Rust is a critical part of writing robust and reliable software. Rust provides powerful tools for handling errors, primarily through the `Result` and `Option` types. Below are some common patterns for error handling in Rust.

### The `Result` Type

The `Result` type is used for functions that can return an error. It is an enum with two variants: `Ok(T)` for successful results and `Err(E)` for errors.

```rust
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

### The `Option` Type

The `Option` type is used for values that may or may not be present. It is an enum with two variants: `Some(T)` for a value and `None` for no value.

```rust
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

### The `?` Operator

The `?` operator is a shorthand for propagating errors. It can be used in functions that return a `Result` or `Option`.

```rust
use std::fs::File;
use std::io::Read;

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

### Custom Error Types

You can define your own error types to provide more context about errors. Below is an example of how to create and use custom error types in Rust.

```rust
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

We define a custom error type `MyError` that can represent both I/O errors and parsing errors. We then implement the `fmt::Display` trait for `MyError` to provide a user-friendly error message. Additionally, we implement the `From` trait to convert from `std::io::Error` and `std::num::ParseIntError` to `MyError`. This allows us to use the `?` operator to propagate these errors in the `read_and_parse_file` function.

These patterns cover the basics of error handling in Rust. By using `Result`, `Option`, the `?` operator, and custom error types, you can write code that gracefully handles errors and provides useful feedback.