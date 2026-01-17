### 堆排序（Heap Sort）

堆排序利用二叉堆实现原地排序，时间复杂度 O(n log n)，空间复杂度 O(1)。在 Rust 中可使用 `binary_heap` 或实现堆操作。

```rust, editable
fn heap_sort(a: &mut [i32]) {
    use std::collections::BinaryHeap;
    let mut heap = BinaryHeap::from(a.to_vec());
    for i in (0..a.len()).rev() { a[i] = heap.pop().unwrap(); }
}
```
