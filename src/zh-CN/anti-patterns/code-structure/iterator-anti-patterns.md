### 不使用迭代器

当迭代器可以直接链接操作时，收集中间结果是低效且冗长的。在操作之间创建临时向量会浪费内存并模糊数据流。当开发人员不熟悉迭代器适配器时，通常会出现这种反模式。

迭代器链是惰性的，这意味着它们只在需要时处理元素，并且编译器可以将整个链优化为单个操作。

```rust, editable
fn process_numbers(numbers: Vec<i32>) -> Vec<String> {
    // 步骤 1：过滤
    let mut filtered = Vec::new();
    for n in &numbers {
        if *n > 0 {
            filtered.push(*n);
        }
    }
    
    // 步骤 2：平方
    let mut squared = Vec::new();
    for n in &filtered {
        squared.push(n * n);
    }
    
    // 步骤 3：转换为字符串
    let mut result = Vec::new();
    for n in &squared {
        result.push(format!("#{}", n));
    }
    
    result
}

fn main() {
    let numbers = vec![-2, -1, 0, 1, 2, 3];
    let result = process_numbers(numbers);
    println!("{:?}", result); // 打印：["#1", "#4", "#9"]
}
```

此代码创建了三个临时向量（`filtered`、`squared`、`result`），每个都需要单独的分配。每个循环独立地遍历数据，导致对集合进行三次单独的遍历。这在时间和空间上都是低效的，并且逻辑分散在多个循环中。

下一个示例使用迭代器链接。

```rust, editable
fn process_numbers(numbers: Vec<i32>) -> Vec<String> {
    numbers.into_iter()
        .filter(|&n| n > 0)
        .map(|n| n * n)
        .map(|n| format!("#{}", n))
        .collect()
}

fn main() {
    let numbers = vec![-2, -1, 0, 1, 2, 3];
    let result = process_numbers(numbers);
    println!("{:?}", result); // 打印：["#1", "#4", "#9"]
}
```

**主要改进**：
- 单次遍历数据 - 无中间集合
- 每个元素在处理下一个元素之前流经整个管道
- 更高的内存效率 - 仅分配最终结果
- 数据转换管道清晰且具有声明性
- 编译器可以将整个链优化为一次操作