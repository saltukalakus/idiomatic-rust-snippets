### 二分搜索

二分搜索是一种从**有序**项目列表中查找项目的高效算法。它的工作原理是重复地将列表中可能包含该项目的部分一分为二，直到将可能的位置缩小到仅一个。

此实现假定输入数组是已排序的。如果数组未排序，二分搜索算法将无法正常工作。请查看[排序部分](../sorting/intro.html)以获取一些数组排序算法的实现。

```rust, editable
fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len().saturating_sub(1);

    while low <= high {
        let mid = low + (high - low) / 2;
        if arr[mid] == target {
            return Some(mid);
        } else if arr[mid] < target {
            low = mid + 1;
        } else {
            high = mid.saturating_sub(1);
        }
    }

    None
}

fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let target = 6;
    match binary_search(&arr, target) {
        Some(index) => println!("在索引 {} 处找到 {}", index, target),
        None => println!("在数组中未找到 {}", target),
    }
}
```

- `binary_search` 函数接受一个整数切片和一个要搜索的目标整数。
- 它初始化两个指针 `low` 和 `high`，分别指向数组的开始和结束。
- 它进入一个循环，计算中间索引并比较中间元素与目标。
- 如果中间元素等于目标，则返回索引。
- 如果中间元素小于目标，它会调整 `low` 指针以搜索数组的右半部分。
- 如果中间元素大于目标，它会调整 `high` 指针以搜索数组的左半部分。
- 如果未找到目标，则返回 `None`。
