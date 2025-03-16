### References

References allow you to refer to some value without taking ownership of it. References are immutable by default, meaning you cannot modify the value they point to. However, you can create mutable references that allow you to change the value.

### Immutable References

An immutable reference is created using the `&` symbol.

```rust
fn main() {
    let x = 5;
    let y = &x; // y is an immutable reference to x

    println!("x: {}, y: {}", x, y);
}
```

`y` is an immutable reference to `x`. You can read the value of `x` through `y`, but you cannot modify it.

### Mutable References

A mutable reference is created using the `&mut` symbol.

```rust
fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x

    *y += 1; // modify the value of x through y

    println!("x: {}", x);
}
```

`y` is a mutable reference to `x`. You can modify the value of `x` through `y`.

### Rules of References

1. At any given time, you can have either one mutable reference or any number of immutable references.
2. References must always be valid.

These rules ensure memory safety and prevent data races at compile time.

### Example with Functions

You can also pass references to functions:

```rust
fn main() {
    let x = 5;
    print_value(&x); // pass an immutable reference to the function

    let mut y = 10;
    increment(&mut y); // pass a mutable reference to the function

    println!("y: {}", y);
}

fn print_value(val: &i32) {
    println!("Value: {}", val);
}

fn increment(val: &mut i32) {
    *val += 1;
}
```

`print_value` takes an immutable reference, and `increment` takes a mutable reference.