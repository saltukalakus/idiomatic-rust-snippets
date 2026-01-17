### 归并排序（Merge Sort）

归并排序是分治算法，将数组分为两半递归排序后合并。时间复杂度 O(n log n)，但需要 O(n) 辅助空间。

```rust, editable
fn merge_sort(a: &mut [i32]) {
    let n = a.len();
    if n <= 1 { return; }
    let mid = n / 2;
    merge_sort(&mut a[..mid]);
    merge_sort(&mut a[mid..]);
    let mut merged = a.to_vec();
    {
        let (left, right) = a.split_at(mid);
        let mut i = 0; let mut l = 0; let mut r = 0;
        while l < left.len() && r < right.len() {
            if left[l] <= right[r] { merged[i] = left[l]; l += 1; }
            else { merged[i] = right[r]; r += 1; }
            i += 1;
        }
        while l < left.len() { merged[i] = left[l]; l += 1; i += 1; }
        while r < right.len() { merged[i] = right[r]; r += 1; i += 1; }
    }
    a.copy_from_slice(&merged);
}
```
