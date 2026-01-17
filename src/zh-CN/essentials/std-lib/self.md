### `self` 参数

在方法定义中，`self` 表示方法的接收者，可以有几种形式：

- `self`：取得所有权并移动值
- `&self`：不可变借用
- `&mut self`：可变借用

```rust, editable
struct Counter { value: i32 }

impl Counter {
    fn consume(self) { /* takes ownership */ }
    fn read(&self) -> i32 { self.value }
    fn inc(&mut self) { self.value += 1; }
}
```

选择哪种形式取决于方法是否需要所有权或可变访问。
