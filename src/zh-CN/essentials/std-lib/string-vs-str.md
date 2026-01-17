### String 与 &str

理解 `String` 与 `&str` 之间的区别在 Rust 中非常重要。两者都表示文本，但在所有权语义、内存特性和使用场景上有所不同。

### 什么是 `&str`？

`&str`（字符串切片）是对 UTF-8 字节序列的不可变引用，它是对存储在其他地方字符串数据的借用视图。

```rust, editable
fn main() {
    // 字符串字面量是 &str
    let greeting: &str = "你好，世界！";
    
    // &str 不拥有数据
    let slice: &str = &greeting[0..5]; // "Hello"
    
    println!("{}", slice);
}
```

### 什么是 `String`？

`String` 是拥有所有权、可增长、在堆上分配的字符串类型。它拥有其数据并且可以被修改。

```rust, editable
fn main() {
    // String 拥有数据
    let mut owned = String::from("Hello");
    
    // 可以修改拥有的字符串
    owned.push_str(", world!");
    
    println!("{}", owned); // "你好，世界！"
}
```

### 关键区别

- **所有权**：`&str` 为借用（引用），`String` 为拥有。
- **内存**：`&str` 为指针 + 长度（通常在栈上），`String` 在堆上分配数据。
- **可变性**：`&str` 不可变，`String` 可变（如果声明为 `mut`）。
- **大小**：`&str` 固定大小，`String` 可增长。
- **成本**：传递 `&str` 开销小，复制 `String` 开销大。
- **生命周期**：`&str` 有生命周期参数，`String` 不需要。

### 何时使用

**使用 `&str`：** 仅需读取字符串、作为函数参数、处理字符串字面量或借用字符串的一部分。

**使用 `String`：** 需要拥有字符串数据、构建或修改字符串、从函数返回字符串或在结构体/集合中存储字符串时。

### 常见转换

```rust, editable
fn main() {
    // &str -> String
    let str_slice: &str = "hello";
    let owned: String = str_slice.to_string();
    let owned2: String = String::from(str_slice);
    let owned3: String = str_slice.to_owned();
    
    // String -> &str（通过 Deref 强制自动完成）
    let owned = String::from("hello");
    let borrowed: &str = &owned;
    let borrowed2: &str = owned.as_str();
    
    println!("{} {}", borrowed, borrowed2);
}
```

### 函数参数

通常优先使用 `&str` 作为函数参数，因为它更灵活：

```rust, editable
// 推荐：接受 &str 和 String
fn print_text(text: &str) {
    println!("{}", text);
}

// 不太灵活：仅接受 String
fn print_text_owned(text: String) {
    println!("{}", text);
}

fn main() {
    let owned = String::from("owned");
    let borrowed = "borrowed";
    
    // 两种类型都可以
    print_text(&owned);     // String 自动解引用为 &str
    print_text(borrowed);   // &str 直接可用
    
    // 仅适用于 String
    print_text_owned(owned.clone());
    // print_text_owned(borrowed); // 错误：需要 String，得到 &str
}
```

### 字符串切片

可以从 `String` 创建 `&str` 切片：

```rust, editable
fn main() {
    let s = String::from("你好，世界！");
    
    // 创建切片
    let hello: &str = &s[0..5];
    let world: &str = &s[7..12];
    
    println!("{} {}", hello, world); // "Hello world"
    
    // 完整切片
    let full: &str = &s[..];
}
```

### 字符串字面量

字符串字面量是 `&'static str`，在程序整个生命周期内有效：

```rust,ignore
fn main() {
    // 这是 &'static str
    let literal: &'static str = "I live forever!";
    
    // 字符串字面量嵌入在二进制中
    let another = "Also static";
}
```

### 构建字符串

```rust, editable
fn main() {
    // 使用 String::new() 与 push_str
    let mut s = String::new();
    s.push_str("Hello");
    s.push(' ');
    s.push_str("world");
    
    // 使用 format! 宏
    let name = "Alice";
    let greeting = format!("你好，{}！", name);
    
    // 使用 + 操作符（移动第一个 String）
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 被移动，s2 被借用
    
    // 使用 +=
    let mut result = String::from("a");
    result += "b";
    result += "c";
    
    println!("{}", result);
}
```

### 内存布局与性能考虑

略（同英文文档）

### 最佳实践

- **函数参数优先使用 `&str`**，更灵活且高效
- **需要拥有数据时使用 `String`**，用于存储或修改字符串
- **避免不必要的 `.to_string()` 调用**，尽量使用 `&str`
- **对于复杂字符串构造使用 `format!`**
- **注意 UTF-8 边界**，切片必须在字符边界上，否则会 panic
