# Does Rust Have a `new` Keyword?

Rust does not have a `new` keyword like some other programming languages (e.g., C++ or Java). Instead, Rust commonly uses an associated function named `new` to create instances of a type. This is a convention rather than a language feature.

Here is an example of how to implement and use a `new` function in Rust:

```rust
struct MyStruct {
    value: i32,
}

impl MyStruct {
    // Associated function `new` to create an instance of `MyStruct`
    fn new(value: i32) -> MyStruct {
        MyStruct { value }
    }
}

fn main() {
    // Creating an instance of `MyStruct` using the `new` function
    let instance = MyStruct::new(10);
    println!("MyStruct value: {}", instance.value);
}
```

In this example, the `new` function is defined as an associated function of `MyStruct` and is used to create a new instance of `MyStruct`.