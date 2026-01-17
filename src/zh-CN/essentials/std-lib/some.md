### `Some` - `Option` 的一种变体

`Some(T)` 表示 `Option<T>` 中存在一个值；通常与 `None` 一并使用来表示可选值：

```rust, editable
fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 { None } else { Some(a / b) }
}

fn main() {
    match divide(10, 2) {
        Some(v) => println!("Result: {}", v),
        None => println!("Divide by zero"),
    }
}
```

`Option` 的常见方法包括 `map`、`and_then`（又名 `flat_map`）、`unwrap_or` 等，能方便地组合可选操作。
