### 快速排序（Quick Sort）

快速排序通过分区将数组划分为两个部分，然后递归排序两部分。平均时间复杂度 O(n log n)，最坏 O(n^2)。

```rust, editable
fn quick_sort(a: &mut [i32]) {
    if a.len() <= 1 { return; }
    let pivot = a[a.len()/2];
    let mut left = Vec::new();
    let mut right = Vec::new();
    let mut pivots = Vec::new();
    for &x in a.iter() {
        if x < pivot { left.push(x); }
        else if x > pivot { right.push(x); }
        else { pivots.push(x); }
    }
    quick_sort(&mut left);
    quick_sort(&mut right);
    let mut i = 0;
    for v in left { a[i] = v; i += 1; }
    for v in pivots { a[i] = v; i += 1; }
    for v in right { a[i] = v; i += 1; }
}
```
