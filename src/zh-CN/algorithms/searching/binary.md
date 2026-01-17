### 二分搜索（Binary Search）

二分搜索用于在有序数组中以对数时间查找元素。实现需注意边界条件以避免无限循环或越界。

```rust, editable
fn binary_search(slice: &[i32], target: i32) -> Option<usize> {
    let mut lo = 0usize;
    let mut hi = slice.len();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if slice[mid] == target { return Some(mid); }
        else if slice[mid] < target { lo = mid + 1; }
        else { hi = mid; }
    }
    None
}

fn main() {
    let a = [1, 3, 5, 7, 9];
    println!("pos: {:?}", binary_search(&a, 5));
}
```
