### `derive(Debug)`

`#[derive(Debug)]` 自动为类型生成实现 `Debug` trait 的代码，允许使用 `{:?}` 或 `{:#?}` 输出调试信息。

```rust, editable
#[derive(Debug)]
struct Person { name: String, age: u8 }

fn main() {
    let p = Person { name: "Alice".into(), age: 30 };
    println!("{:?}", p);
}
```

当你的类型包含无法自动实现 `Debug` 的字段时，你可以手动实现 `Debug` 或为字段选择性地实现 `#[derive(Debug)]`。
