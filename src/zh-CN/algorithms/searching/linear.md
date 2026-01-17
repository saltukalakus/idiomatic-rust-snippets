### 线性搜索算法

线性搜索是一种简单的搜索算法，它按顺序检查列表中的每个元素，直到找到所需的元素或列表结束。

```rust, editable
fn linear_search(arr: &[i32], target: i32) -> Option<usize> {
    for (index, &item) in arr.iter().enumerate() {
        if item == target {
            return Some(index);
        }
    }
    None
}

fn main() {
    let numbers = [1, 3, 5, 7, 9, 11];
    let target = 7;

    match linear_search(&numbers, target) {
        Some(index) => println!("在索引 {} 处找到 {}", index, target),
        None => println!("在数组中未找到 {}", target),
    }
}
```

- `linear_search` 函数接受一个整数切片 (`arr`) 和一个目标整数 (`target`) 作为参数。
- 该函数使用 `enumerate()` 遍历数组，该方法同时提供每个元素的索引和值。对于每个元素，它检查该元素是否等于目标。如果找到匹配项，则返回包装在 `Some` 中的索引。如果循环完成而未找到目标，则函数返回 `None`。
