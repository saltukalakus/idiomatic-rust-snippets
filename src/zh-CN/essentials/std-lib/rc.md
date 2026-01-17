### 理解 `Rc`

`Rc`（引用计数）用于在单线程环境中实现多所有权。它允许程序的多个部分共享同一数据的只读所有权，而无需复制数据。

1- 使用 `Rc::new` 创建引用计数的值实例。

2- 使用 `Rc::clone` 创建指向相同数据的新引用，这会增加引用计数，从而允许多个所有者共享数据。

`Rc` 不是线程安全的，仅适用于单线程场景。多线程场景请使用 `Arc`（原子引用计数）。

#### `Rc::new`

```rust, editable
use std::rc::Rc;

fn main() {
    let value = Rc::new(5);
    println!("值: {}", value);
}
```

- `Rc::new(5)` 创建一个持有值 `5` 的 `Rc` 实例。

#### `Rc::clone`

```rust, editable
use std::rc::Rc;

fn main() {
    let value = Rc::new(5);
    let value_clone = Rc::clone(&value);

    println!("值: {}", value);
    println!("克隆的值: {}", value_clone);
}
```

- `Rc::clone(&value)` 创建指向相同数据的新引用。`value` 和 `value_clone` 指向相同的数据，引用计数会增加。