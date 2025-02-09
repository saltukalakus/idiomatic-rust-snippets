### Understanding `Rc` in Rust

The `Rc` (Reference Counted) type in Rust is used when you need multiple ownership of data. It enables multiple parts of your program to read from the same data without needing to copy it. However, `Rc` is not thread-safe, so it should only be used in single-threaded scenarios.

### `Rc::new`

The `Rc::new` function is used to create a new reference-counted instance of a value.

```rust
use std::rc::Rc;

fn main() {
    let value = Rc::new(5);
    println!("Value: {}", value);
}
```

`Rc::new(5)` creates a new `Rc` instance that holds the value `5`.

### `Rc::clone`

The `Rc::clone` function is used to create a new reference to the same data. This increases the reference count, allowing multiple parts of your program to share ownership of the data.

```rust
use std::rc::Rc;

fn main() {
    let value = Rc::new(5);
    let value_clone = Rc::clone(&value);

    println!("Value: {}", value);
    println!("Cloned Value: {}", value_clone);
}
```

`Rc::clone(&value)` creates a new reference to the same data. Both `value` and `value_clone` point to the same data, and the reference count is increased.

### Summary

- `Rc::new` is used to create a new reference-counted instance of a value.
- `Rc::clone` is used to create a new reference to the same data, increasing the reference count.

Remember, `Rc` is for single-threaded scenarios. For multi-threaded scenarios, consider using `Arc` (Atomic Reference Counted) instead.