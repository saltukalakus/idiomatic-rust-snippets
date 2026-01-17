### `#[derive(Debug)]` 的用途是什么？

Rust 中的 `#[derive(Debug)]` 属性会自动为结构体或枚举生成 `Debug` trait 的实现。该 trait 允许您使用 `{:?}` 格式化程序来格式化值，这对于调试非常有用。

```rust, editable
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    // 使用 Debug trait 打印 person 结构体
    println!("{:?}", person);
}
```

`Person` 结构体派生了 `Debug` trait，允许我们使用 `println!("{:?}", person);` 打印其值。输出将是：

```
Person { name: "Alice", age: 30 }
```

这使得在开发和调试期间检查复杂数据结构的值变得更加容易。
