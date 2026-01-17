### 理解 `RefCell`

`RefCell` 是一种在 Rust 中提供内部可变性的类型。它允许你在有不可变引用的情况下修改数据。这是通过在运行时而不是编译时强制执行借用规则来实现的。

- `RefCell` 是 `std::cell` 模块的一部分。
- 它启用了在运行时检查的可变借用。
- 在你需要修改数据但只有一个不可变引用的场景中很有用。

```rust, editable
{{#include refcell/src/main.rs}}
```

1. 我们创建一个包含值 `5` 的 `RefCell`。
2. 我们使用 `borrow()` 不可变地借用值。
3. 我们使用 `borrow_mut()` 可变地借用值并修改它。
4. 我们再次不可变地借用值以验证更改。

- 如果你违反了借用规则（例如，在存在不可变借用的情况下尝试可变借用），`RefCell` 将在运行时引发 panic。
- 当你需要内部可变性并且确定在运行时会遵守借用规则时，请使用 `RefCell`。
- `RefCell` 在单线程中管理值时有效。对于多线程场景，请使用 `Mutex` 或 `RwLock`。

更多详情，请参阅 [Rust `RefCell` 文档](https://doc.rust-lang.org/std/cell/struct.RefCell.html)。
