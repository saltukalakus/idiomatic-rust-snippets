### String 与 str 的混淆

在只需读取字符串时使用 `String` 会导致不必要的分配。`String` 是拥有所有权的堆分配缓冲区，而 `&str` 是对字符串数据的借用视图。许多函数仅读取字符串并不需要所有权，因此接受 `String` 会迫使调用方克隆或分配。

除非需要所有权，否则优先在函数参数中使用 `&str`，这会使 API 更灵活且高效。

```rust, editable
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

fn is_valid_email(email: String) -> bool {
    email.contains('@') && email.contains('.')
}

fn main() {
    let user = "Alice".to_string();
    
    // 由于 greet 接受所有权，必须克隆
    let greeting = greet(user.clone());
    println!("{}", greeting);
    
    // 对 is_valid_email 也需要克隆
    let email = "alice@example.com".to_string();
    if is_valid_email(email.clone()) {
        println!("Valid email: {}", email);
    }
    
    // 使用字符串字面量需要分配
    greet("Bob".to_string()); // 不必要的分配
}
```

每次函数调用都需要克隆或分配新的 `String`。`greet(user.clone())` 会为 "Alice" 分配一个副本。`"Bob".to_string()` 也会进行分配来传递字面量。呼叫频繁时，这会带来分配压力并浪费 CPU 时间。

下面的示例在读取参数上使用 `&str`。

```rust, editable
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn is_valid_email(email: &str) -> bool {
    email.contains('@') && email.contains('.')
}

fn main() {
    let user = "Alice".to_string();
    
    // 无需克隆 —— 借用即可
    let greeting = greet(&user);
    println!("{}", greeting);
    
    // 可以重复使用 user，无需克隆
    let email = "alice@example.com".to_string();
    if is_valid_email(&email) {
        println!("Valid email: {}", email);
    }
    
    // 字符串字面量可直接使用 —— 无需分配
    greet("Bob");
}
```

**关键改进**：
- 函数接受 `&str` —— 可处理借用数据、字符串字面量和 `String` 的引用
- 无不必要的分配 —— 无需防御性克隆
- API 更加灵活 —— 接受 `&String`、`&str`、字面量和切片
- 调用方保留所有权 —— 可多次使用字符串
- 符合 Rust 惯例 —— 读取使用 `&str`，需要所有权时使用 `String`

