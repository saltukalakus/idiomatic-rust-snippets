### Ownership

Ownership is a set of rules that governs how the program manages memory. The ownership system is designed to prevent memory leaks and ensure memory safety. Here are the key rules of ownership:

1. Each value has a variable that’s called its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2

    // println!("{}", s1); // This line would cause a compile-time error because s1 is no longer valid
    println!("{}", s2); // s2 is now the owner of the string
}
```
