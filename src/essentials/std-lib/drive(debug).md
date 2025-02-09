### What is the purpose of `#[derive(Debug)]`?

The `#[derive(Debug)]` attribute in Rust automatically generates an implementation of the `Debug` trait for a struct or enum. This trait allows you to format the value using the `{:?}` formatter, which is useful for debugging purposes.

```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    // Print the person struct using the Debug trait
    println!("{:?}", person);
}
```

The `Person` struct derives the `Debug` trait, allowing us to print its value using `println!("{:?}", person);`. The output will be:

```
Person { name: "Alice", age: 30 }
```

This makes it easier to inspect the values of complex data structures during development and debugging.