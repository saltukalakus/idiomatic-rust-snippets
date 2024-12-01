### Understanding `RefCell` in Rust

`RefCell` is a type that provides interior mutability in Rust. It allows you to mutate data even when there are immutable references to that data. This is achieved by enforcing borrowing rules at runtime rather than compile time.

### Key Points

- `RefCell` is part of the `std::cell` module.
- It enables mutable borrowing checked at runtime.
- Useful in scenarios where you need to mutate data but only have an immutable reference.

#### Example

Here's a simple example to illustrate how `RefCell` works:

```rust
{{#include refcell/src/main.rs}}
```

### Explanation

1. We create a `RefCell` containing the value `5`.
2. We borrow the value immutably using `borrow()`.
3. We borrow the value mutably using `borrow_mut()` and modify it.
4. We borrow the value immutably again to verify the change.

### Important Notes

- `RefCell` will panic at runtime if you violate borrowing rules (e.g., if you try to borrow mutably while an immutable borrow is active).
- Use `RefCell` when you need interior mutability and are sure that the borrowing rules will be followed at runtime.
- `RefCell` works when the value is managed in a single thread. For multi-threaded scenarios use Mutex or RwLock.

For more details, refer to the [Rust documentation on `RefCell`](https://doc.rust-lang.org/std/cell/struct.RefCell.html).