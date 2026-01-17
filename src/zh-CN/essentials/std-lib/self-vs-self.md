### `Self` vs `self` 的区别

- `Self`：在类型或 trait 上下文中，表示实现块的类型本身（类型级别）。
- `self`：方法接收者实例（值级别）。

```rust,ignore
struct MyStruct;

impl MyStruct {
    // 关联函数，使用 `Self`
    fn new() -> Self {
        MyStruct
    }

    // 方法，使用 `self`
    fn consume(self) {
        // 消费实例
    }
}
```

- `Self` 在 `new` 函数中用于指代 `MyStruct` 类型。
- `self` 在 `consume` 方法中用于指代 `MyStruct` 的实例。