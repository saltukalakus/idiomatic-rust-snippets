### 快速排序算法

快速排序基于将数据数组划分为更小的数组。一个大数组被划分为两个数组，其中一个包含小于指定值（比如基准）的值，另一个数组包含大于基准值的值。

1.  从数组中选择一个元素，称为基准。
2.  将数组划分为两个子数组：
    *   小于基准的元素。
    *   大于基准的元素。
3.  递归地将上述步骤应用于子数组。

```rust, editable
fn quick_sort(arr: &mut [i32]) {
     let len = arr.len();
     if len < 2 {
          return;
     }
     let pivot_index = partition(arr);
     println!("左侧数组: {:?}", &arr[0..pivot_index]);
     println!("右侧数组: {:?}", &arr[pivot_index + 1..len]);
     quick_sort(&mut arr[0..pivot_index]);
     quick_sort(&mut arr[pivot_index + 1..len]);
}

fn partition(arr: &mut [i32]) -> usize {
     let len = arr.len();

     // 选择中间元素作为基准
     let pivot_index = len / 2;
     arr.swap(pivot_index, len - 1);
     
     let mut store_index = 0;
     for i in 0..len - 1 {
          if arr[i] < arr[len - 1] {
                arr.swap(i, store_index);
                store_index += 1;
          }
     }
     arr.swap(store_index, len - 1);
     store_index
}

fn main() {
     let mut arr = [64, 34, 25, 12, 22, 12, 90, 33];
     println!("原始数组: {:?}", arr);
     quick_sort(&mut arr);
     println!("排序后的数组: {:?}", arr);
}
```

- `quick_sort` 函数：此函数接受一个可变的整数切片，并使用快速排序算法对其进行排序。它递归地对子数组进行排序。
- `partition` 函数：此函数围绕一个基准元素对数组进行分区，并返回分区后基准的索引。如果数组已经半排序，选择中间项作为基准通常更好。
