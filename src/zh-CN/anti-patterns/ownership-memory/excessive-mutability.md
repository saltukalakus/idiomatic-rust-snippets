### 过度可变

将变量标记为 `mut` 而实际上不需要变更是常见的反模式。过度可变会使代码更难理解，容易引入意外修改，并阻碍编译器优化。Rust 默认不可变是一个特性，而非限制。

不可变性传达了意图：该值不会改变，使代码更可预测、更易于调试。

```rust, editable
fn calculate_price(base_price: f64, quantity: u32) -> f64 {
    let mut price = base_price;
    let mut qty = quantity;
    let mut tax_rate = 0.08;
    
    let subtotal = price * qty as f64;
    tax_rate = 0.1; // 意外修改 —— 本应保持 0.08
    
    subtotal * (1.0 + tax_rate)
}

fn main() {
    let total = calculate_price(10.0, 5);
    println!("总价: ${:.2}", total); // 输出: $55.00 (错误，应为 $54.00)
    
    let expected = 10.0 * 5.0 * 1.08;
    println!("期望值: ${:.2}", expected); // 输出: $54.00
}
```

这三个变量都被标记为 `mut`，但实际上没有必要进行可变操作。`tax_rate` 被意外重新赋值从 `0.08` 变为 `0.1`，但是编译器不会阻止这种操作，从而导致错误的计算。这个 bug 很容易在代码审查中被忽视。

下面的示例通过默认不可变来改进。

```rust, editable
fn calculate_price(base_price: f64, quantity: u32) -> f64 {
    let price = base_price;
    let qty = quantity;
    let tax_rate = 0.08;
    
    let subtotal = price * qty as f64;
    // tax_rate = 0.1; // 编译错误：不能对不可变变量再次赋值
    
    subtotal * (1.0 + tax_rate)
}

fn main() {
    let total = calculate_price(10.0, 5);
    println!("总价: ${:.2}", total); // 输出: $54.00 (正确)
    
    let expected = 10.0 * 5.0 * 1.08;
    println!("期望值: ${:.2}", expected); // 输出: $54.00
}
```

**关键改进**：
- 默认不可变 —— 防止意外修改
- 若尝试重新赋值，编译器会报错
- 意图更明确 —— 读者知道这些值不会改变
- 更易重构 —— 无需跟踪所有修改点
- 启用编译器优化 —— 不可变值可以被更激进地优化

