### 迭代器反模式

不当使用迭代器会导致不必要的临时分配、两次遍历或复杂的可读性问题。优先考虑使用迭代器链（`iter().map().filter().collect()`）来表达逻辑，而不是手动索引或显式循环管理状态。

反模式示例：手动索引并在循环中进行条件处理：

```rust, editable
let v = vec![1, 2, 3, 4];
let mut doubled = Vec::new();
for i in 0..v.len() {
    if v[i] % 2 == 0 {
        doubled.push(v[i] * 2);
    }
}
```

更好的写法：

```rust, editable
let doubled: Vec<_> = v.iter().filter(|&&x| x % 2 == 0).map(|&x| x * 2).collect();
```

当需要索引时，考虑使用 `enumerate()` 而不是手工管理索引变量。