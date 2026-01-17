### `Self` vs `self` 的区别

- `Self`：在类型或 trait 上下文中，表示实现块的类型本身（类型级别）。
- `self`：方法接收者实例（值级别）。

```rust, editable
struct Foo;

impl Foo {
    fn new() -> Self { Foo }

    fn consume(self) { /* 使用 self */ }
}
```

记住 `Self` 是类型，而 `self` 是实例。两者在不同语法位置使用。