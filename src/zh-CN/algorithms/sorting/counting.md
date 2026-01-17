### 计数排序算法

计数排序是一种整数排序算法，它通过计算具有不同键值的对象的数量来进行操作。它不是一种基于比较的排序算法，当输入值的范围不显著大于要排序的对象的数量时，它很有效。有关该算法的可视化解释，请访问[Programiz](https://www.programiz.com/dsa/counting-sort)。

1.  确定输入数组中的最小值和最大值。
2.  创建一个大小等于输入值范围的数组，并初始化为零。
3.  遍历输入数组并计算每个值的出现次数，将计数存储在计数数组中。
4.  修改计数数组，使每个索引处的每个元素存储先前计数的总和。此步骤有助于将元素放置在输出数组中的正确位置。
5.  按相反顺序遍历输入数组，根据计数数组将每个元素放置在输出数组中的正确位置，然后将计数减一。

```rust, editable
fn counting_sort(arr: &mut [usize], max_value: usize) {
    let mut count = vec![0; max_value + 1];
    let mut output = vec![0; arr.len()];

    // 计算每个值的出现次数
    for &num in arr.iter() {
        count[num] += 1;
    }
    println!("计数数组: {:?}", count);

    // 累积计数
    for i in 1..=max_value {
        count[i] += count[i - 1];
    }
    println!("累积计数数组: {:?}", count);

    // 构建输出数组
    for &num in arr.iter().rev() {
        println!("正在处理的当前数字: {}", num);
        output[count[num] - 1] = num;
        println!("输出: {:?}", output);
        count[num] -= 1;
    }

    // 将排序后的元素复制回原始数组
    arr.copy_from_slice(&output);
}

fn main() {
    let mut arr = [4, 2, 2, 8, 3, 3, 1];
    let max_value = *arr.iter().max().unwrap();
    counting_sort(&mut arr, max_value);
    println!("排序后的数组: {:?}", arr);
}
```

- `counting_sort` 函数接受一个可变的整数切片 `arr` 和数组中的最大值 `max_value`。
- 它初始化一个 `count` 数组来存储每个唯一值的计数，以及一个 `output` 数组来存储排序后的元素。
- 它遍历输入数组以计算每个值的出现次数。
- 它修改 `count` 数组以累积计数。
- 它按相反顺序遍历输入数组，以将每个元素放置在 `output` 数组中的正确位置。
- 最后，它将排序后的元素从 `output` 数组复制回原始数组。

此实现假定输入数组包含非负整数，并且最大值是已知的或可以确定的。
