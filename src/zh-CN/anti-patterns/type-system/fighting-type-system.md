### 与类型系统对抗（Fighting the Type System）

使用类型转换、transmute 或 `as` 强制转换类型以“凑合”类型，是导致 bug 的常见做法。Rust 的类型系统可以在编译时防止许多错误。与之对抗（使用不安全转换或忽略类型不匹配）会引入在运行时才暴露的问题。

当类型系统阻止你时，通常意味着设计有问题。应与类型协作，而不是对抗它们。

```rust, editable
fn calculate_average(numbers: Vec<i32>) -> i32 {
    let sum: i32 = numbers.iter().sum();
    let count = numbers.len() as i32;
    sum / count // 当 count 为 0 时会 panic
}

fn process_bytes(data: &[u8]) -> u32 {
    // 危险：假设 data 至少有 4 个字节
    unsafe {
        let ptr = data.as_ptr() as *const u32;
        *ptr // 若 data.len() < 4 则为未定义行为
    }
}

fn main() {
    let numbers = vec![10, 20, 30];
    let avg = calculate_average(numbers);
    println!("Average: {}", avg); // Prints: Average: 20
    
    let empty: Vec<i32> = vec![];
    // 这会 panic：thread 'main' panicked at 'attempt to divide by zero'
    let avg = calculate_average(empty);
    println!("Average: {}", avg);
}
```

`calculate_average` 在空向量时会 panic（除以零）。函数签名承诺返回 `i32`，但实际上并不总是可用。使用 `as i32` 强制转换只是绕过类型问题而不是修复它们。`process_bytes` 使用 unsafe 将字节转为 `u32`，若长度不足则会导致未定义行为。

下面的示例使用更合适的类型。

```rust, editable
fn calculate_average(numbers: &[i32]) -> Option<f64> {
    if numbers.is_empty() {
        return None;
    }
    
    let sum: i32 = numbers.iter().sum();
    let count = numbers.len() as f64;
    Some(sum as f64 / count)
}

fn process_bytes(data: &[u8]) -> Option<u32> {
    if data.len() < 4 {
        return None;
    }
    
    let bytes: [u8; 4] = [data[0], data[1], data[2], data[3]];
    Some(u32::from_le_bytes(bytes))
}

fn main() {
    let numbers = vec![10, 20, 30];
    if let Some(avg) = calculate_average(&numbers) {
        println!("Average: {:.2}", avg); // Prints: Average: 20.00
    }
    
    let empty: Vec<i32> = vec![];
    match calculate_average(&empty) {
        Some(avg) => println!("Average: {:.2}", avg),
        None => println!("Cannot calculate average of empty list"), // Prints this
    }
}
```

**关键改进**：
- 返回 `Option<f64>` —— 函数签名反映了平均值可能不存在
- 使用 `f64` 表示平均值 —— 整数除法会丢失精度
- 无 panic —— 错误情况以 `None` 返回
- `process_bytes` 在转换前检查长度
- 使用 `u32::from_le_bytes()` 代替不安全的 transmute

