### 冒泡排序算法

冒泡排序是一种简单的排序算法，它重复地遍历列表，比较相邻的元素，如果它们的顺序错误就交换它们。重复遍历列表，直到列表排序完成。有关该算法的可视化解释，请访问[维基百科](https://en.wikipedia.org/wiki/Bubble_sort)。

```rust, editable
fn bubble_sort(arr: &mut [i32]) {
    let mut n = arr.len();
    while n > 0 {
        let mut new_n = 0;
        for i in 1..n {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                new_n = i;
            }
        }
        println!("迭代 {:?} {}", arr, new_n);
        n = new_n;
    }
}

fn main() {
    let mut arr = [64, 34, 25, 12, 22, 12, 90, 33];
    println!("未排序的数组: {:?}", arr);
    bubble_sort(&mut arr);
    println!("排序后的数组: {:?}", arr);
}
```

- `bubble_sort` 函数接受一个可变的整数切片。
- 外部的 `while` 循环一直持续到数组排序完成。变量 `n` 跟踪数组未排序部分的长度。
- 内部的 `for` 循环遍历数组，比较相邻的元素。
- 如果位置 `i-1` 的元素大于位置 `i` 的元素，则使用 `swap` 方法交换它们。
- 变量 `new_n` 跟踪最后一次交换的位置，这有助于减少后续遍历中的比较次数。
