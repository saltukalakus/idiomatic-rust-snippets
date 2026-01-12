### String vs &str

Understanding the difference between `String` and `&str` is crucial in Rust. Both represent text, but they have different ownership semantics, memory characteristics, and use cases.

### What is &str?

`&str` (string slice) is an immutable reference to a sequence of UTF-8 encoded bytes. It's a borrowed view into string data stored elsewhere.

```rust
fn main() {
    // String literals are &str
    let greeting: &str = "Hello, world!";
    
    // &str does not own the data
    let slice: &str = &greeting[0..5]; // "Hello"
    
    println!("{}", slice);
}
```

### What is String?

`String` is an owned, growable, heap-allocated string type. It owns its data and can be modified.

```rust
fn main() {
    // String owns the data
    let mut owned = String::from("Hello");
    
    // Can modify owned strings
    owned.push_str(", world!");
    
    println!("{}", owned); // "Hello, world!"
}
```

### Key Differences

| Feature | `&str` | `String` |
|---------|--------|----------|
| **Ownership** | Borrowed (reference) | Owned |
| **Memory** | Stack (pointer + length) | Heap-allocated |
| **Mutability** | Immutable | Can be mutable |
| **Size** | Fixed size | Growable |
| **Cost** | Cheap to pass around | Copying is expensive |
| **Lifetime** | Has a lifetime parameter | No lifetime needed |

### When to Use Each

**Use `&str` when:**
- You only need to read the string
- Accepting string parameters in functions
- Working with string literals
- You want to borrow part of a string

**Use `String` when:**
- You need to own the string data
- Building or modifying strings
- Returning strings from functions
- Storing strings in structs or collections

### Common Conversions

```rust
fn main() {
    // &str to String
    let str_slice: &str = "hello";
    let owned: String = str_slice.to_string();
    let owned2: String = String::from(str_slice);
    let owned3: String = str_slice.to_owned();
    
    // String to &str (automatic via Deref coercion)
    let owned = String::from("hello");
    let borrowed: &str = &owned;
    let borrowed2: &str = owned.as_str();
    
    println!("{} {}", borrowed, borrowed2);
}
```

### Function Parameters

Always prefer `&str` for function parameters - it's more flexible:

```rust
// Good: accepts both &str and String
fn print_text(text: &str) {
    println!("{}", text);
}

// Less flexible: only accepts String
fn print_text_owned(text: String) {
    println!("{}", text);
}

fn main() {
    let owned = String::from("owned");
    let borrowed = "borrowed";
    
    // Works with both types
    print_text(&owned);     // String automatically derefs to &str
    print_text(borrowed);   // &str works directly
    
    // Only works with String
    print_text_owned(owned.clone());
    // print_text_owned(borrowed); // Error: expected String, found &str
}
```

### String Slicing

You can create `&str` slices from `String`:

```rust
fn main() {
    let s = String::from("Hello, world!");
    
    // Create slices
    let hello: &str = &s[0..5];
    let world: &str = &s[7..12];
    
    println!("{} {}", hello, world); // "Hello world"
    
    // Full slice
    let full: &str = &s[..];
}
```

### String Literals

String literals in your code are `&'static str` - they live for the entire program:

```rust,ignore
fn main() {
    // This is a &'static str
    let literal: &'static str = "I live forever!";
    
    // String literals are embedded in the binary
    let another = "Also static";
}
```

### Building Strings

```rust
fn main() {
    // Using String::new() and push_str
    let mut s = String::new();
    s.push_str("Hello");
    s.push(' ');
    s.push_str("world");
    
    // Using format! macro
    let name = "Alice";
    let greeting = format!("Hello, {}!", name);
    
    // Using + operator (consumes first String)
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 is moved, s2 is borrowed
    
    // Using concatenation
    let mut result = String::from("a");
    result += "b";
    result += "c";
    
    println!("{}", result);
}
```

### Memory Layout

```rust
fn main() {
    // &str: Just a pointer and length (stack)
    let str_slice: &str = "hello";
    // Memory: [ptr: 8 bytes, len: 8 bytes] = 16 bytes on stack
    
    // String: Pointer, length, and capacity (stack) + data (heap)
    let string: String = String::from("hello");
    // Memory: [ptr: 8 bytes, len: 8 bytes, cap: 8 bytes] = 24 bytes on stack
    //         + actual string data on heap
    
    println!("&str size: {} bytes", std::mem::size_of::<&str>());
    println!("String size: {} bytes", std::mem::size_of::<String>());
}
```

### Performance Considerations

```rust,ignore
fn main() {
    // Efficient: No allocation
    let s: &str = "hello";
    
    // Allocation required
    let owned = String::from("hello");
    
    // Cloning &str is cheap (just copies pointer+length)
    let s2 = s;
    
    // Cloning String is expensive (copies heap data)
    let owned2 = owned.clone();
}
```

### Common Patterns

**Returning strings from functions:**

```rust
// Return &str when returning string literals or slices
fn get_greeting() -> &'static str {
    "Hello!"
}

// Return String when building new strings
fn get_full_name(first: &str, last: &str) -> String {
    format!("{} {}", first, last)
}

fn main() {
    println!("{}", get_greeting());
    println!("{}", get_full_name("John", "Doe"));
}
```

**Storing in structs:**

```rust,ignore
// Use &str with lifetime annotation
struct BorrowedName<'a> {
    name: &'a str,
}

// Use String when struct owns the data
struct OwnedName {
    name: String,
}

fn main() {
    let borrowed = BorrowedName { name: "Alice" };
    let owned = OwnedName { name: String::from("Bob") };
}
```

### Best Practices

- **Prefer `&str` for function parameters** - more flexible and efficient
- **Use `String` for ownership** - when you need to store or modify the string
- **Avoid unnecessary `.to_string()` calls** - use `&str` when possible
- **Use `format!`** for complex string building instead of repeated concatenation
- **Remember string literals are `&str`** - no allocation needed
- **Use `.as_str()` explicitly** when clarity is needed, but Deref coercion usually works
- **Be mindful of UTF-8** - slicing must be on character boundaries or it will panic
