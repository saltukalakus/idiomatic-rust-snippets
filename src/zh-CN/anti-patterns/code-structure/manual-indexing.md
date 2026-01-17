### 手动索引（Manual Indexing）

直接使用整数索引访问集合（例如 `v[i]`）并在循环中手动管理索引变量通常更容易出错——特别是在插入/删除或并发变更的场景中。Rust 的迭代器和切片方法通常能以更安全、表达性更强的方式完成同样的任务。

反模式：

```rust, editable
let mut i = 0;
while i < v.len() {
    process(v[i]);
    i += 1;
}
```

更好的替代：

```rust, editable
for item in &v {
    process(item);
}
```

需要索引信息时使用 `enumerate()`：

```rust, editable
for (i, item) in v.iter().enumerate() {
    println!("{}: {}", i, item);
}
```
