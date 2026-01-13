### Borrowing

Borrowing allows you to reference a value without taking ownership of it. There are two types of borrowing in Rust:

**Immutable Borrowing (&T)**: 

You can have multiple immutable references to a value.

```rust, editable
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // Borrow s1 immutably

    println!("The length of '{}' is {}.", s1, len); // s1 is still valid here
}

fn calculate_length(s: &String) -> usize {
    s.len() // Use the borrowed value
}
```

s1 is borrowed immutably by the calculate_length function. The function can read the value but cannot modify it. After the function call, s1 is still valid.

**Mutable Borrowing (&mut T)**: 

You can have only one mutable reference to a value at a time. 

```rust, editable
fn main() {
    let mut s = String::from("hello");
    change(&mut s); // Borrow s mutably

    println!("{}", s); // s is still valid here and has been modified
}

fn change(some_string: &mut String) {
    some_string.push_str(", world"); // Modify the borrowed value
}
```
s is borrowed mutably by the change function. The function can modify the value. After the function call, s is still valid and has been modified.
