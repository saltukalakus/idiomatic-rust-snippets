### String vs str Confusion

Using `String` when `&str` would suffice causes unnecessary allocations. `String` is an owned, heap-allocated buffer, while `&str` is a borrowed view into string data. Many functions only read strings and don't need ownership, so accepting `String` forces callers to clone or allocate.

Prefer `&str` in function parameters unless you need ownership. This makes your API more flexible and efficient.

```rust
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

fn is_valid_email(email: String) -> bool {
    email.contains('@') && email.contains('.')
}

fn main() {
    let user = "Alice".to_string();
    
    // Must clone because greet takes ownership
    let greeting = greet(user.clone());
    println!("{}", greeting);
    
    // Must clone again for is_valid_email
    let email = "alice@example.com".to_string();
    if is_valid_email(email.clone()) {
        println!("Valid email: {}", email);
    }
    
    // Using string literals requires allocation
    greet("Bob".to_string()); // Unnecessary allocation
}
```

Every function call requires cloning or allocating a new `String`. `greet(user.clone())` allocates a copy of "Alice". `"Bob".to_string()` allocates just to pass a literal. These functions only read the strings but take ownership, forcing defensive cloning. With many calls, this creates allocation pressure and wastes CPU time.

The next sample uses `&str` for read-only parameters.

```rust
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn is_valid_email(email: &str) -> bool {
    email.contains('@') && email.contains('.')
}

fn main() {
    let user = "Alice".to_string();
    
    // No cloning needed - borrows work
    let greeting = greet(&user);
    println!("{}", greeting);
    
    // Can use user again without cloning
    let email = "alice@example.com".to_string();
    if is_valid_email(&email) {
        println!("Valid email: {}", email);
    }
    
    // String literals work directly - no allocation
    greet("Bob");
}
```

**Key Improvements**:
- Functions accept `&str` - work with borrowed data, string literals, and `String` references
- Zero unnecessary allocations - no defensive cloning required
- More flexible API - accepts `&String`, `&str`, string literals, and slices
- Callers retain ownership - can use the string multiple times
- Idiomatic Rust - use `&str` for reading, `String` for ownership
