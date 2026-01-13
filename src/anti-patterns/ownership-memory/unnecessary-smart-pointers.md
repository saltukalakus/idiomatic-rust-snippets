### Unnecessary Smart Pointers

Using `Box<T>`, `Rc<T>`, or `Arc<T>` when simple references or owned values work is wasteful. Smart pointers add indirection and heap allocation overhead. They're designed for specific ownership scenarios: recursive types, shared ownership, or dynamic trait objects.

Using smart pointers unnecessarily complicates code and degrades performance. Prefer direct ownership or references unless you have a specific reason for indirection.

```rust, editable
struct Config {
    timeout: Box<u32>,      // Unnecessary heap allocation
    max_retries: Box<u32>,  // Unnecessary heap allocation
    endpoint: Box<String>,  // Double heap allocation
}

impl Config {
    fn new() -> Self {
        Config {
            timeout: Box::new(30),
            max_retries: Box::new(3),
            endpoint: Box::new("https://api.example.com".to_string()),
        }
    }
    
    fn display(&self) {
        println!("Timeout: {}s", self.timeout);
        println!("Retries: {}", self.max_retries);
        println!("Endpoint: {}", self.endpoint);
    }
}

fn main() {
    let config = Config::new();
    config.display();
    // Three unnecessary heap allocations for simple values
}
```

Boxing primitive types like `u32` adds heap allocation overhead for no benefit. `Box<String>` creates double indirection since `String` is already a heap-allocated type. Each `Box::new()` call requires a separate heap allocation. The memory layout becomes fragmented, reducing cache efficiency and increasing allocation overhead.

The next sample uses direct ownership.

```rust, editable
struct Config {
    timeout: u32,
    max_retries: u32,
    endpoint: String,
}

impl Config {
    fn new() -> Self {
        Config {
            timeout: 30,
            max_retries: 3,
            endpoint: "https://api.example.com".to_string(),
        }
    }
    
    fn display(&self) {
        println!("Timeout: {}s", self.timeout);
        println!("Retries: {}", self.max_retries);
        println!("Endpoint: {}", self.endpoint);
    }
}

fn main() {
    let config = Config::new();
    config.display();
    // Only one heap allocation for the String
}
```

**Key Improvements**:
- Primitive types stored directly - no heap allocation
- Better memory locality - all fields except String are stack-allocated
- Simpler code - no unnecessary `Box::new()` or dereferencing
- Faster access - no pointer indirection for primitives
- Smaller struct size - `Config` is now cache-friendly
