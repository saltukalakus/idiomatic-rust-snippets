### 归并排序算法

归并排序是一种分治算法，它将数组分成两半，递归地对每一半进行排序，然后将已排序的两半合并以产生最终的排序数组。有关该算法的可视化解释，请访问[Programiz](https://www.programiz.com/dsa/merge-sort)。


1.  将数组分成两半。
2.  递归地对每一半进行排序。
3.  将两个已排序的半部分合并成一个单一的排序数组。

```rust, editable
fn merge_sort<T: Ord + Clone>(arr: &mut [T]) {
    let mid = arr.len() / 2;
    if mid == 0 {
        return;
    }

    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);

    let mut ret = arr.to_vec();

    merge(&arr[..mid], &arr[mid..], &mut ret[..]);

    arr.clone_from_slice(&ret);
}

// 将两个子数组 left 和 right 合并到 ret 数组中
fn merge<T: Ord + Clone>(left: &[T], right: &[T], ret: &mut [T]) {
    let mut left_iter = left.iter();
    let mut right_iter = right.iter();
    let mut left_peek = left_iter.next();
    let mut right_peek = right_iter.next();
    let mut i = 0;

    // 在到达 left 或 right 数组的任一端之前，选取
    // 元素中较小的一个，并将其放置在返回数组的正确位置
    while let (Some(left_val), Some(right_val)) = (left_peek, right_peek) {
        if left_val <= right_val {
            ret[i] = left_val.clone();
            left_peek = left_iter.next();
        } else {
            ret[i] = right_val.clone();
            right_peek = right_iter.next();
        }
        i += 1;
    }

    // 此时，left 或 right 数组中可能还有元素
    // 尚未克隆到返回的数组中，这些元素
    // 接下来被克隆。

    while let Some(left_val) = left_peek {
        ret[i] = left_val.clone();
        left_peek = left_iter.next();
        i += 1;
    }

    while let Some(right_val) = right_peek {
        ret[i] = right_val.clone();
        right_peek = right_iter.next();
        i += 1;
    }
}

fn main() {
    let mut arr = vec![38, 27, 43, 3, 9, 82, 10];
    println!("原始数组: {:?}", arr);
    merge_sort(&mut arr);
    println!("排序后的数组: {:?}", arr);
}
```

- `merge_sort` 函数：递归地将数组分成两半并对每一半进行排序。
- `merge` 函数：将两个已排序的切片合并成一个单一的已排序切片。
