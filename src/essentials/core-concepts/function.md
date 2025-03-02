### Functions in Rust

Functions are a fundamental building block in Rust. They allow you to encapsulate code into reusable blocks. Functions in Rust are defined using the `fn` keyword, followed by the function name, parameters, and the body of the function.

### Defining a Function

```rust
fn main() {
    println!("Hello, world!");
}
```

`main` is a special function that serves as the entry point of a Rust program.

### Function Parameters

Functions can take parameters, which are specified in the parentheses after the function name.

```rust
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    greet("Alice");
    greet("Bob");
}
```

The `greet` function takes a single parameter `name` of type `&str`.

### Return Values

Functions can also return values. The return type is specified after an arrow (`->`).

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let sum = add(5, 3);
    println!("Sum: {}", sum);
}
```

The `add` function takes two parameters of type `i32` and returns their sum, which is also of type `i32`.

### Early Returns

You can return a value early from a function using the `return` keyword:

```rust
fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    }
    false
}

fn main() {
    let number = 4;
    if is_even(number) {
        println!("{} is even", number);
    } else {
        println!("{} is odd", number);
    }
}
```

The `is_even` function returns `true` if the number is even and `false` otherwise.