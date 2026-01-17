### 计数排序（Counting Sort）

计数排序适用于元素范围较小且为整数的情况，通过统计每个值出现次数来线性排序。时间复杂度 O(n + k)，其中 k 是值域大小。

```rust, editable
fn counting_sort(a: &mut [usize], max_value: usize) {
    let mut counts = vec![0usize; max_value + 1];
    for &v in a.iter() { counts[v] += 1; }
    let mut idx = 0;
    for v in 0..=max_value {
        for _ in 0..counts[v] {
            a[idx] = v; idx += 1;
        }
    }
}
```
