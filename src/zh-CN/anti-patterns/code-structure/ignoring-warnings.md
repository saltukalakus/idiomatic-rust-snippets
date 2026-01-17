### 忽略编译器警告

Rust 的编译器提供了有用的警告，指出潜在的错误、性能问题或非惯用代码。忽略这些警告通常会导致本可以及早发现的细微错误。虽然抑制警告以“使代码干净地编译”可能很诱人，但这通常会掩盖根本问题，而不是解决它们。

常见的警告场景包括未使用的变量、未使用的 `Result` 值（静默忽略错误）、无法访问的代码和非详尽的模式匹配。

```rust, editable
fn calculate_total(prices: Vec<&str>) -> i32 {
    let mut total = 0;
    
    for price_str in prices {
        price_str.parse::<i32>(); // 警告：必须使用的未使用 Result
        total += 10; // 盲目地增加 10，忽略实际的解析结果
    }
    
    total
}

fn main() {
    let prices = vec!["25", "invalid", "30", "15"];
    let total = calculate_total(prices);
    println!("总计：${}", total); // 打印：总计：$40（应为 $70 或错误）
}
```

编译器警告 `parse()` 返回一个必须使用的 `Result`，但我们忽略了它。当解析失败时（如“invalid”），程序会静默地继续，增加任意值而不是实际价格。最终的总数完全错误，但程序没有检测到错误。

下一个示例修复了此问题并避免了编译器警告。

```rust, editable
fn calculate_total(prices: Vec<&str>) -> Result<i32, String> {
    let mut total = 0;
    
    for price_str in prices {
        let price = price_str.parse::<i32>()
            .map_err(|_| format!("无效价格：'{}'", price_str))?;
        total += price;
    }
    
    Ok(total)
}

fn main() {
    let prices = vec!["25", "invalid", "30", "15"];
    
    match calculate_total(prices) {
        Ok(total) => println!("总计：${}", total),
        Err(e) => println!("错误：{}", e), // 打印：错误：无效价格：'invalid'
    }
}
```

**主要改进**：
- 函数返回 `Result<i32, String>` 以传播错误
- 捕获解析错误并将其转换为有意义的错误消息
- 立即检测到无效数据：打印 `错误：无效价格：'invalid'`
- 调用者收到正确的总数或清晰的错误消息
- 将数据更改为有效值，如 `vec!["25", "30", "15"]`，将正确打印 `总计：$70`