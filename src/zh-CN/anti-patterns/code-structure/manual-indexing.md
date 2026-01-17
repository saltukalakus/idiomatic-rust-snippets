### 手动索引

Rust 提供了强大的迭代器方法，使代码比手动基于索引的循环更具表现力、更安全，并且通常更高效。使用 `for i in 0..length` 进行手动索引是冗长的，容易出错（可能会因越界访问而恐慌），并且错过了迭代器提供的优化机会。

迭代器也更好地组合，允许你以清晰、函数式的风格链接 `.filter()`、`.map()` 和 `.collect()` 等操作。

```rust, editable
fn sum_adjacent_pairs(numbers: Vec<i32>) -> Vec<i32> {
    let mut sums = Vec::new();
    
    for i in 0..numbers.len() {
        let sum = numbers[i] + numbers[i + 1]; // 当 i = numbers.len() - 1 时会恐慌
        sums.push(sum);
    }
    
    sums
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let sums = sum_adjacent_pairs(numbers);
    println!("相邻对的和: {:?}", sums);
}
```

使用 `numbers[i + 1]` 进行手动索引会导致运行时恐慌。当 `i` 到达最后一个索引时，`i + 1` 超出范围。这是手动索引中常见的错误——编译器无法捕获的差一错误。程序会因 `thread 'main' panicked at 'index out of bounds'` 而崩溃。

下一个示例使用惯用的迭代器方法来避免此问题。

```rust, editable
fn sum_adjacent_pairs(numbers: Vec<i32>) -> Vec<i32> {
    numbers.windows(2)
        .map(|pair| pair[0] + pair[1])
        .collect()
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let sums = sum_adjacent_pairs(numbers);
    println!("相邻对的和: {:?}", sums); // 打印：相邻对的和: [3, 5, 7, 9]
}
```

**主要改进**：
- `.windows(2)` 方法安全地提供 2 个元素的滑动窗口
- 不会恐慌——迭代器会自动正确处理边界
- 更简洁、更具表现力——清楚地表明我们正在处理相邻对
- 逻辑在构造上是正确的——不可能出现差一错误
- 可组合——可以链接其他操作而无需索引管理
