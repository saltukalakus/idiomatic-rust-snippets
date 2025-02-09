### Does Rust Have a `new` Keyword?

Rust does **not** have a `new` keyword like some other programming languages (e.g., C++ or Java). Instead, Rust commonly uses an associated function named `new` to create instances of a type. This is a convention rather than a language feature.

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

The `new` function is defined as an associated function of `MyStruct` and is used to create a new instance of `MyStruct`.