### Unwrap Abuse

Calling `.unwrap()` or `.expect()` on `Result` and `Option` types turns recoverable errors into panics. This is acceptable in examples, tests, or prototypes, but production code should handle errors gracefully. Unwrapping forces the calling code to deal with panics instead of errors.

Rust's type system makes errors explicit with `Result<T, E>`. Unwrapping throws away this safety and turns Rust into a language that crashes like C.

```rust, editable
use std::fs;
use std::env;

fn read_config() -> String {
    let path = env::var("CONFIG_PATH").unwrap(); // Panics if env var not set
    let contents = fs::read_to_string(&path).unwrap(); // Panics if file doesn't exist
    contents
}

fn main() {
    let config = read_config();
    println!("Config loaded: {} bytes", config.len());
}
```

This code panics if `CONFIG_PATH` isn't set or if the file doesn't exist. The panic message is generic: `thread 'main' panicked at 'called Option::unwrap() on a None value'`. No context about what failed or why. The program crashes instead of allowing recovery or providing useful feedback. Users see a cryptic panic instead of an error message.

The next sample uses proper error propagation.

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
            println!("Config loaded: {} bytes", config.len());
        }
        Err(e) => {
            eprintln!("Failed to load config: {}", e);
            eprintln!("Please set CONFIG_PATH environment variable");
            std::process::exit(1);
        }
    }
}
```

**Key Improvements**:
- Returns `Result` instead of panicking - errors can be handled by caller
- Uses `?` operator for clean error propagation
- Provides helpful error messages to users instead of panic backtraces
- Allows graceful shutdown with `exit(1)` instead of panic
- Caller can choose how to handle errors - retry, use defaults, or exit
