### Panic in Libraries

Libraries should never panic in normal operation - they should return `Result` types instead. When a library panics, it forces that panic onto all users of the library. Applications should decide how to handle errors, not libraries. Panicking violates the principle of least surprise.

A library that panics is saying "I know better than you how to handle this error" - this is almost never true.

```rust
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

The library panics on invalid input (`"0"` or `"invalid"`), crashing the entire application. The application can't recover, log the error, use a default value, or skip the invalid port. Library code takes control away from the application developer. The program crashes with: `thread 'main' panicked at 'Port must be greater than 0'`.

The next sample returns `Result` instead of panicking.

```rust
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

**Key Improvements**:
- Returns `Result<u16, String>` - errors are values, not crashes
- Application continues running even with invalid input
- Caller decides the error handling strategy - skip, use default, retry, or exit
- Errors provide context that can be logged or shown to users
- Library follows Rust conventions - fallible operations return `Result`
