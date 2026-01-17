### 迭代器（Iterator）模式

Rust 的迭代器是构建在 trait 基础上的强大抽象，提供惰性计算、组合器方法（`map`、`filter`、`fold`）以及高效的消耗式处理。

```rust, editable
let v = vec![1, 2, 3];
let sum: i32 = v.iter().sum();
println!("和: {}", sum);
```

在需要自定义迭代行为时，实现 `Iterator` trait：

```rust, editable
struct Counter { count: u32 }

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count <= 5 { Some(self.count) } else { None }
    }
}
```
