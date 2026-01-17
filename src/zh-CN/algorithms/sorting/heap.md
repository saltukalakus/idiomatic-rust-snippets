### 堆排序算法

堆排序是一种基于比较的排序算法，它使用[二叉堆数据结构](https://www.programiz.com/dsa/heap-data-structure)。它将其输入划分为一个已排序区域和一个未排序区域，并通过提取最大元素并将其移动到已排序区域来迭代地缩小未排序区域。有关该算法的可视化解释，请访问[Programiz](https://www.programiz.com/dsa/heap-sort)。

1.  从输入数据构建一个最大堆。
2.  此时，最大的项存储在堆的根部。将其与堆的最后一项交换，然后将堆的大小减一。最后，对树的根进行堆化。
3.  当堆的大小大于 1 时，重复步骤 2。

```rust, editable
fn heapify(arr: &mut [i32], n: usize, i: usize) {
    // 找到根、左子节点和右子节点中的最大值
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left < n && arr[left] > arr[largest] {
        largest = left;
    }

    if right < n && arr[right] > arr[largest] {
        largest = right;
    }

    // 如果根不是最大值，则交换并继续堆化
    if largest != i {
        arr.swap(i, largest);
        heapify(arr, n, largest);
    }
}

fn heap_sort(arr: &mut [i32]) {
    let n = arr.len();

    // 构建最大堆
    for i in (0..n / 2).rev() {
        heapify(arr, n, i);
    }

    // 此时 arr[0] 保存着最大的项
    println!("最大堆数组: {:?}", arr);

    // 堆排序
    for i in (1..n).rev() {
        arr.swap(0, i);
        // 堆化根元素以再次在根部获取最高元素
        heapify(arr, i, 0);
    } 
} 

fn main() {
    let mut arr = [64, 34, 25, 12, 22, 12, 90, 33];
    println!("原始数组: {:?}", arr);
    heap_sort(&mut arr);
    println!("排序后的数组: {:?}", arr);
}
```

- `heapify` 函数确保以索引 `i` 为根的子树是一个最大堆。它将根与其子节点进行比较，并在必要时交换它们，然后递归地堆化受影响的子树。
- `heap_sort` 函数首先从输入数组构建一个最大堆。然后，它重复地从堆中提取最大元素并将其放置在数组的末尾，每次都减小堆的大小。
