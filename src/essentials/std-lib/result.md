### Result Type

The `Result` type is Rust's primary mechanism for handling operations that can fail. It forces explicit error handling at compile time, preventing the silent propagation of errors common in other languages.

### Definition

The `Result` enum is defined in the standard library as:

```rust,ignore
enum Result<T, E> {
    Ok(T),    // Success case containing a value of type T
    Err(E),   // Error case containing an error of type E
}
```

### Basic Usage

```rust
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

### Handling Results with `?` Operator

The `?` operator provides concise error propagation. It returns early with the error if the `Result` is `Err`, or unwraps the `Ok` value:

```rust
fn parse_number(s: &str) -> Result<i32, String> {
    s.parse::<i32>()
        .map_err(|e| format!("Failed to parse '{}': {}", s, e))
}

fn add_two_numbers(a: &str, b: &str) -> Result<i32, String> {
    let num_a = parse_number(a)?;  // Returns error if parse fails
    let num_b = parse_number(b)?;  // Returns error if parse fails
    Ok(num_a + num_b)
}

fn main() {
    // Success case
    match add_two_numbers("10", "20") {
        Ok(sum) => println!("Sum: {}", sum),
        Err(e) => println!("Error: {}", e),
    }
    
    // Error case
    match add_two_numbers("10", "abc") {
        Ok(sum) => println!("Sum: {}", sum),
        Err(e) => println!("Error: {}", e),
    }
}
```

### Common Result Methods

**Checking the variant**:
- `is_ok()`: Returns `true` if `Ok`
- `is_err()`: Returns `true` if `Err`

**Extracting values**:
- `unwrap()`: Returns the `Ok` value or panics (use sparingly!)
- `expect(msg)`: Like `unwrap()` but with custom panic message
- `unwrap_or(default)`: Returns value or a default
- `unwrap_or_else(f)`: Returns value or computes default from closure

```rust
fn main() {
    let good_result: Result<i32, &str> = Ok(10);
    let bad_result: Result<i32, &str> = Err("error");
    
    // Checking
    println!("Is ok: {}", good_result.is_ok());  // true
    println!("Is err: {}", bad_result.is_err()); // true
    
    // Extracting with defaults
    println!("{}", good_result.unwrap_or(0));     // 10
    println!("{}", bad_result.unwrap_or(0));      // 0
    
    println!("{}", bad_result.unwrap_or_else(|e| {
        eprintln!("Error occurred: {}", e);
        -1
    })); // -1
}
```

### Transforming Results

**`map()` and `map_err()`**: Transform the contained value or error

```rust
fn main() {
    let result: Result<i32, &str> = Ok(2);
    
    // Transform the Ok value
    let doubled = result.map(|x| x * 2);
    println!("{:?}", doubled); // Ok(4)
    
    // Transform the Err value
    let err: Result<i32, &str> = Err("oops");
    let mapped_err = err.map_err(|e| format!("Error: {}", e));
    println!("{:?}", mapped_err); // Err("Error: oops")
}
```

**`and_then()`**: Chain operations that return Results

```rust
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

### Collecting Results

When working with iterators of Results, you can collect them into a single Result:

```rust
fn parse_numbers(strings: Vec<&str>) -> Result<Vec<i32>, std::num::ParseIntError> {
    strings.into_iter()
        .map(|s| s.parse::<i32>())
        .collect()  // Collects Vec<Result<i32, E>> into Result<Vec<i32>, E>
}

fn main() {
    match parse_numbers(vec!["1", "2", "3"]) {
        Ok(numbers) => println!("Numbers: {:?}", numbers),
        Err(e) => eprintln!("Parse error: {}", e),
    }
}
```

### Custom Error Types

For complex applications, define custom error types:

```rust
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

### Result vs Option

- Use `Result<T, E>` when you need to provide error information
- Use `Option<T>` when absence of a value is a valid, expected case without error details

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

### Converting Between Result and Option

```rust
fn main() {
    let result: Result<i32, &str> = Ok(5);
    let option: Option<i32> = result.ok();  // Ok(5) -> Some(5)
    
    let result2: Result<i32, &str> = option.ok_or("missing value");  // Some(5) -> Ok(5)
    
    println!("{:?}", result2);
}
```

### Best Practices

- **Use `?` operator** for clean error propagation in functions returning `Result`
- **Avoid `unwrap()`** in production code - use `expect()` with descriptive messages or proper error handling
- **Return `Result` from library functions** - let callers decide how to handle errors
- **Use custom error types** for domain-specific error handling
- **Leverage `map()`, `and_then()`** for transforming and chaining operations
- **Consider using error handling crates** like `anyhow` or `thiserror` for complex applications
