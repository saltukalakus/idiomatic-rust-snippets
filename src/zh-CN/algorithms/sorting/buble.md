### 冒泡排序（Bubble Sort）

冒泡排序是最简单的排序算法之一，重复交换相邻逆序元素。时间复杂度为 O(n^2)，通常仅用于教学或小规模数据。

```rust, editable
fn bubble_sort(a: &mut [i32]) {
    let n = a.len();
    for i in 0..n {
        for j in 0..n - 1 - i {
            if a[j] > a[j+1] { a.swap(j, j+1); }
        }
    }
}
```
