### Borrow vs Ownership in Rust

Let's expand on the concepts of borrowing and ownership in Rust. These are fundamental concepts in Rust's memory management system, which ensures memory safety without needing a garbage collector.

### Ownership

Ownership is a set of rules that governs how a Rust program manages memory. The ownership system is designed to prevent memory leaks and ensure memory safety. Here are the key rules of ownership:

1. Each value in Rust has a variable that’s called its owner.
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


### Borrowing

Borrowing allows you to reference a value without taking ownership of it. There are two types of borrowing in Rust:

Immutable Borrowing (&T): You can have multiple immutable references to a value.

In this example, s1 is borrowed immutably by the calculate_length function. The function can read the value but cannot modify it. After the function call, s1 is still valid.

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // Borrow s1 immutably

    println!("The length of '{}' is {}.", s1, len); // s1 is still valid here
}

fn calculate_length(s: &String) -> usize {
    s.len() // Use the borrowed value
}
```

Mutable Borrowing (&mut T): You can have only one mutable reference to a value at a time.

In this example, s is borrowed mutably by the change function. The function can modify the value. After the function call, s is still valid and has been modified.

```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s); // Borrow s mutably

    println!("{}", s); // s is still valid here and has been modified
}

fn change(some_string: &mut String) {
    some_string.push_str(", world"); // Modify the borrowed value
}
```

