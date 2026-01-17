### `Self` 作为类型

在 `impl` 块或 trait 中，`Self` 可用来指代实现块中的类型本身：

```rust, editable
struct Builder;

impl Builder {
    fn build() -> Self { Builder }
}

trait Cloneable { fn clone_box(&self) -> Box<Self>; }
```

在 trait 定义中使用 `Self` 可以帮助编写与类型无关的抽象。
