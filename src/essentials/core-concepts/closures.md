### Closures in Rust

Closures are anonymous functions that can capture their environment. They are similar to lambdas in other programming languages. Closures in Rust are defined using the `|` syntax.

### Basic Syntax

```rust
let add = |a, b| a + b;
println!("Sum: {}", add(2, 3)); // Output: Sum: 5
```

### Capturing Environment

Closures can capture variables from their enclosing scope.

```rust
let x = 10;
let print_x = || println!("x: {}", x);
print_x(); // Output: x: 10
```

### Mutable Captures

Closures can also capture variables mutably.

```rust
let mut x = 10;
{
    let mut add_to_x = || x += 5;
    add_to_x();
}
println!("x: {}", x); // Output: x: 15
```

### Moving Captures

Closures can take ownership of captured variables using the `move` keyword.

```rust
let x = vec![1, 2, 3];
let print_x = move || println!("x: {:?}", x);
print_x(); // Output: x: [1, 2, 3]
// x is no longer accessible here
```

Closures are a powerful feature in Rust, enabling concise and expressive code.