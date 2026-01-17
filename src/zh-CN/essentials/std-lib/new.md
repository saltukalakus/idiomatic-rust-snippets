### Rust 有 `new` 关键字吗？

Rust **没有**像其他一些编程语言（例如 C++ 或 Java）那样的 `new` 关键字。相反，Rust 通常使用一个名为 `new` 的关联函数来创建类型的实例。这是一种约定，而不是语言特性。

```rust, editable
struct MyStruct {
    value: i32,
}

impl MyStruct {
    // 关联函数 `new`，用于创建 `MyStruct` 的实例
    fn new(value: i32) -> MyStruct {
        MyStruct { value }
    }
}

fn main() {
    // 使用 `new` 函数创建 `MyStruct` 的实例
    let instance = MyStruct::new(10);
    println!("MyStruct 的值: {}", instance.value);
}
```

- `new` 函数被定义为 `MyStruct` 的关联函数，用于创建 `MyStruct` 的新实例。
