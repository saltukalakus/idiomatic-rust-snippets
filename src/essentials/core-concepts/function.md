### Functions in Rust

Functions are a fundamental building block in Rust. They allow you to encapsulate code into reusable blocks. Functions in Rust are defined using the `fn` keyword, followed by the function name, parameters, and the body of the function.

### Defining a Function

Here is a simple example of a function in Rust:

```rust
fn main() {
    println!("Hello, world!");
}
```

In this example, `main` is a special function that serves as the entry point of a Rust program.

### Function Parameters

Functions can take parameters, which are specified in the parentheses after the function name. Here is an example:

```rust
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    greet("Alice");
    greet("Bob");
}
```

In this example, the `greet` function takes a single parameter `name` of type `&str`.

### Return Values

Functions can also return values. The return type is specified after an arrow (`->`). Here is an example:

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let sum = add(5, 3);
    println!("Sum: {}", sum);
}
```

In this example, the `add` function takes two parameters of type `i32` and returns their sum, which is also of type `i32`.

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

In this example, the `is_even` function returns `true` if the number is even and `false` otherwise.

### Conclusion

Functions in Rust are powerful tools for organizing and reusing code. They can take parameters, return values, and even return early if needed. Understanding how to define and use functions is essential for writing idiomatic Rust code.