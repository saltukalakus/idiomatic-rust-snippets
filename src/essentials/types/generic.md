### What is generics?

Generics in Rust allow you to write flexible and reusable code by enabling you to define functions, [structs](./struct.md), [enums](./enum.md), and [traits](./trait.md) that can operate on many different types without sacrificing performance. Generics are a powerful feature that helps you write more abstract and type-safe code.

### Key Concepts of Generics

Generic Functions can operate on different types specified at the time of function call.

In the example, the **larger** function is defined with a generic type parameter `T`. The type `T` must implement the `PartialOrd` trait (for comparison) and the `Copy` trait (so we can return it by value). The function compares two values and returns the larger one.

```rust
fn larger<T: PartialOrd + Copy>(x: T, y: T) -> T {
    if x > y {
        x
    } else {
        y
    }
}

fn main() {
    let a = 10;
    let b = 20;
    let result = larger(a, b);
    println!("The larger value is: {}", result);  // Prints: 20

    let c = 5.5;
    let d = 2.3;
    let result = larger(c, d);
    println!("The larger value is: {}", result);  // Prints: 5.5
}
```

Structs that can hold data of different types specified at the time of instantiation.

Here the Point struct has been implemented to work with both integer and floating type.

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };

    println!("Integer Point: ({}, {})", integer_point.x, integer_point.y);
    println!("Float Point: ({}, {})", float_point.x, float_point.y);
}
```

Enums that can hold variants of different types specified at the time of instantiation.

Here's a custom generic enum similar to the standard library's `Option<T>`:

```rust
#[derive(Debug)]
enum MyOption<T> {
    Some(T),
    None,
}

fn main() {
    let some_number = MyOption::Some(5);
    let some_string = MyOption::Some("a string");
    let no_value: MyOption<i32> = MyOption::None;

    println!("Some number: {:?}", some_number);
    println!("Some string: {:?}", some_string);
    println!("No value: {:?}", no_value);
}
```

**Note**: Rust's standard library provides `Option<T>` with the same structure, which is automatically available in every Rust program.