### The `Some` Keyword 

In Rust, the `Some` keyword is used to represent a value within the [`Option`](./option.md) type. The `Option` type is an enum that can either be `Some(T)` where `T` is a value, or `None`, indicating the absence of a value. This is useful for handling cases where a value might be optional.

#### Example

```rust
fn main() {
    let some_number = Some(5);
    let some_string = Some("Hello");

    if let Some(value) = some_number {
        println!("We have a number: {}", value);
    }

    if let Some(text) = some_string {
        println!("We have a string: {}", text);
    }
}
```

`some_number` and `some_string` are both [`Option`](./option.md) types. The `if let` syntax is used to check if they contain a value (`Some`) and to extract that value.

### Usage

The `Option` type is widely used in Rust to handle cases where a value might be present or absent, providing a safer alternative to null values found in other languages.
