### Stringly Typed Code

Using strings to represent data that has a fixed set of values is fragile and error-prone. Typos, case sensitivity issues, and invalid values can't be caught at compile time. String-based types bypass Rust's exhaustive pattern matching and type safety.

Enums make invalid states unrepresentable. If your data has a fixed set of values, use an enum.

```rust, editable
fn process_status(status: &str) -> String {
    match status {
        "success" => "Operation completed".to_string(),
        "warning" => "Operation completed with warnings".to_string(),
        "error" => "Operation failed".to_string(),
        _ => "Unknown status".to_string(), // Catches typos, but at runtime
    }
}

fn set_log_level(level: &str) {
    match level {
        "debug" | "info" | "warn" | "error" => {
            println!("Log level set to: {}", level);
        }
        _ => println!("Invalid log level: {}", level), // Runtime error
    }
}

fn main() {
    println!("{}", process_status("success"));
    println!("{}", process_status("sucess")); // Typo - prints "Unknown status"
    
    set_log_level("info");
    set_log_level("Info"); // Case sensitivity - prints "Invalid log level"
    set_log_level("verbose"); // Invalid value - prints "Invalid log level"
}
```

String parameters accept any value - typos like `"sucess"` or invalid values like `"verbose"` compile successfully but fail at runtime. Case sensitivity causes bugs: `"Info"` vs `"info"`. The catch-all `_` branch hides problems instead of preventing them. Adding a new valid status requires updating all match statements, with no compiler help to find them all.

The next sample uses enums for type safety.

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
    // println!("{}", process_status(Status::Sucess)); // Compile error - variant doesn't exist
    
    set_log_level(LogLevel::Info);
    // set_log_level(LogLevel::Verbose); // Compile error - variant doesn't exist
    // No case sensitivity issues - Status::SUCCESS doesn't exist
}
```

**Key Improvements**:
- Typos become compile errors instead of runtime bugs
- Impossible to pass invalid values - only defined variants exist
- No case sensitivity issues - enum variants have exact names
- Exhaustive matching - compiler ensures all cases are handled
- If new variants are added, all match statements must be updated (compile error if not)
