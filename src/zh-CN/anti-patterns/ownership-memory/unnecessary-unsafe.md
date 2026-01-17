### 不必要的 unsafe

当存在安全的 Rust 替代方案时使用 `unsafe` 块违背了 Rust 的安全保证。Unsafe 代码需要手动验证不变式是否被维护。开发者常常过早使用 `unsafe` 来绕过借用检查器或追求所谓的性能提升。

Unsafe 应该作为最后手段。大多数性能关键代码并不需要它，安全抽象通常已经足够高效。

```rust, editable
fn get_middle_element(data: &Vec<i32>) -> Option<i32> {
    if data.is_empty() {
        return None;
    }
    
    let mid = data.len() / 2;
    unsafe {
        // 不必要的 unsafe —— 如果逻辑有误可能会 panic
        Some(*data.get_unchecked(mid))
    }
}

fn main() {
    let numbers = vec![10, 20, 30, 40, 50];
    if let Some(mid) = get_middle_element(&numbers) {
        println!("中间值: {}", mid); // 输出: 中间值: 30
    }
    
    let empty: Vec<i32> = vec![];
    if let Some(mid) = get_middle_element(&empty) {
        println!("中间值: {}", mid);
    } else {
        println!("空向量"); // 输出: 空向量
    }
}
```

使用 `get_unchecked()` 绕过了边界检查，假定程序员已验证索引有效。若 `is_empty()` 检查被误删或索引计算错误，将导致未定义行为。这个 unsafe 块没有带来可观的性能收益，因为边界检查开销微小，但它引入了潜在的内存不安全性。

下面的示例使用安全索引。

```rust, editable
fn get_middle_element(data: &Vec<i32>) -> Option<i32> {
    let mid = data.len() / 2;
    data.get(mid).copied()
}

fn main() {
    let numbers = vec![10, 20, 30, 40, 50];
    if let Some(mid) = get_middle_element(&numbers) {
        println!("中间值: {}", mid); // 输出: 中间值: 30
    }
    
    let empty: Vec<i32> = vec![];
    if let Some(mid) = get_middle_element(&empty) {
        println!("空向量"); // 输出: 空向量
    } else {
        println!("没有中间元素");
    }
}
```

**关键改进**：
- 使用 `.get()` 返回 `Option<&T>` —— 不会 panic
- 自动处理空向量并返回 `None`
- 无需手动边界检查 —— 安全 API 已经处理
- 使用 `.copied()` 安全地解引用并复制值
- 编译器可以验证正确性 —— 无需维护 unsafe 不变式

