### Poor Pattern Matching

Pattern matching in Rust is exhaustive by nature, but using catch-all patterns too early or ignoring specific cases can lead to bugs. Using `_` or catch-all matches when specific cases should be handled causes logic errors and missed edge cases.

Exhaustive pattern matching is one of Rust's strengths - the compiler ensures all cases are handled. Overly broad patterns bypass this safety feature.

```rust, editable
enum Status {
    Success(String),
    Warning(String),
    Error(String),
}

fn process_status(status: Status) -> String {
    match status {
        Status::Success(msg) => format!("OK: {}", msg),
        _ => "Something happened".to_string(), // Too broad - loses information
    }
}

fn main() {
    let warning = Status::Warning("Low memory".to_string());
    let error = Status::Error("Connection failed".to_string());
    
    println!("{}", process_status(warning)); // Prints: Something happened
    println!("{}", process_status(error));   // Prints: Something happened
}
```

The catch-all pattern `_` swallows important information. Warnings and errors produce identical output, making debugging impossible. If a new variant is added to the enum later, this code won't fail to compile - it will silently mishandle the new case. The program runs but produces useless output for critical states.

The next sample uses exhaustive pattern matching.

```rust, editable
enum Status {
    Success(String),
    Warning(String),
    Error(String),
}

fn process_status(status: Status) -> String {
    match status {
        Status::Success(msg) => format!("✓ Success: {}", msg),
        Status::Warning(msg) => format!("⚠ Warning: {}", msg),
        Status::Error(msg) => format!("✗ Error: {}", msg),
    }
}

fn main() {
    let warning = Status::Warning("Low memory".to_string());
    let error = Status::Error("Connection failed".to_string());
    
    println!("{}", process_status(warning)); // Prints: ⚠ Warning: Low memory
    println!("{}", process_status(error));   // Prints: ✗ Error: Connection failed
}
```

**Key Improvements**:
- Each enum variant is explicitly handled with appropriate logic
- Output preserves all information from the original status
- If a new variant is added, the code won't compile until it's handled
- The compiler enforces exhaustiveness, preventing silent failures
- Clear, distinct output for each status type aids debugging
