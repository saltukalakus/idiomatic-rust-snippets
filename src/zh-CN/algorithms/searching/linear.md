### 线性搜索（Linear Search）

线性搜索逐项检查序列来查找目标，适用于未排序或规模较小的数据结构。复杂度为 O(n)。

```rust, editable
fn linear_search(slice: &[i32], target: i32) -> Option<usize> {
    for (i, &v) in slice.iter().enumerate() {
        if v == target { return Some(i); }
    }
    None
}

fn main() {
    let a = [2, 4, 6, 8];
    println!("位置：{:?}", linear_search(&a, 6));
}
```
