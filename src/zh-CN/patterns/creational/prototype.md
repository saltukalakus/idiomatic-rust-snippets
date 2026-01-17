### 原型（Prototype）模式

原型模式通过复制现有对象来创建新实例。Rust 中可以通过实现 `Clone` trait 并使用 `clone()` 来表现类似行为。

```rust, editable
#[derive(Clone)]
struct Thing { data: String }

fn main() {
    let a = Thing { data: "x".into() };
    let b = a.clone();
}
```
