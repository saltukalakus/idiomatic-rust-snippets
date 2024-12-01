#### Option Enum

The `Option` enum in Rust is a powerful feature that represents a value that can either be something (`Some`) or nothing (`None`). It is commonly used to handle cases where a value might be absent, avoiding the need for null pointers and reducing the risk of null pointer exceptions.

### Definition

The `Option` enum is defined in the standard library as follows:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

Here, `T` is a generic type parameter, meaning `Option` can hold a value of any type.

### Usage

#### Example 1: Basic Usage

```rust
fn find_word(word: &str) -> Option<usize> {
    let words = vec!["hello", "world", "rust"];
    words.iter().position(|&w| w == word)
}

fn main() {
    match find_word("rust") {
        Some(index) => println!("Found at index: {}", index),
        None => println!("Not found"),
    }
}
```

In this example, the `find_word` function returns an `Option<usize>`. If the word is found, it returns `Some(index)`, otherwise it returns `None`.

#### Example 2: Using `unwrap`

```rust
fn main() {
    let some_number = Some(10);
    let number = some_number.unwrap();
    println!("The number is: {}", number);
}
```

The `unwrap` method extracts the value inside `Some`, but it will panic if called on a `None` value. Use it only when you are sure that the `Option` is `Some`.

#### Example 3: Using `unwrap_or`

```rust
fn main() {
    let some_number = Some(10);
    let none_number: Option<i32> = None;

    println!("The number is: {}", some_number.unwrap_or(0));
    println!("The number is: {}", none_number.unwrap_or(0));
}
```

The `unwrap_or` method provides a default value if the `Option` is `None`.

#### Example 4: Using `map`

```rust
fn main() {
    let some_number = Some(10);
    let none_number: Option<i32> = None;

    let new_number = some_number.map(|n| n + 1);
    let new_none = none_number.map(|n| n + 1);

    println!("New number: {:?}", new_number);
    println!("New none: {:?}", new_none);
}
```

The `map` method applies a function to the value inside `Some`, if it exists, and returns a new `Option`.