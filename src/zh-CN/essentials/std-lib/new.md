### `new` - 常见构造函数约定

在 Rust 中，`new` 常被用作类型的构造函数（关联函数），返回 `Self` 的实例：

```rust, editable
struct Thing { value: i32 }

impl Thing {
    fn new(value: i32) -> Self {
        Self { value }
    }
}

fn main() {
    let t = Thing::new(42);
    println!("{}", t.value);
}
```

注意：并非所有类型都使用 `new`——有时会使用 `default()`（`Default` trait）或更语义化的构造函数名，例如 `with_capacity`。
