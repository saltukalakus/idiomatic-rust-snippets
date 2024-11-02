### What is generics?

Generics in Rust allow you to write flexible and reusable code by enabling you to define functions, [structs](./struct.md), [enums](./enum.md), and [traits](./trait.md) that can operate on many different types without sacrificing performance. Generics are a powerful feature that helps you write more abstract and type-safe code.

### Key Concepts of Generics

**Generic Functions**: 

Generic Functions can operate on different types specified at the time of function call.

In the example, the **first** function is defined with a generic type parameter T. This means that T can be any type. The function takes two parameters, x and _y, both of type T. The function returns the first parameter x of type T. In the main function, the first function is called with different types of arguments (integers and strings), demonstrating its flexibility.

```rust
fn first<T>(x: T, _y: T) -> T {
    x
}

fn main() {
    let a = 10;
    let b = 20;
    let result = first(a, b);
    println!("The first value is: {}", result);

    let c = "hello";
    let d = "world";
    let result = first(c, d);
    println!("The first value is: {}", result);
}
```

**Generic Structs**: 

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

** Generic Enums**: 

Enums that can hold variants of different types specified at the time of instantiation.

```rust
enum Option<T> {
    Some(T),
    None,
}

fn main() {
    let some_number = Option::Some(5);
    let some_string = Option::Some("a string");

    println!("Some number: {:?}", some_number);
    println!("Some string: {:?}", some_string);
}
```