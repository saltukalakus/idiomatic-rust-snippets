### String vs &str

理解 `String` 和 `&str` 之间的区别在 Rust 中至关重要。它们都代表文本，但具有不同的所有权语义、内存特性和使用场景。

### 什么是 &str？

`&str`（字符串切片）是对一个 UTF-8 编码字节序列的不可变引用。它是对存储在别处的字符串数据的借用视图。

```rust, editable
fn main() {
    // 字符串字面量是 &str
    let greeting: &str = "你好, 世界!";
    
    // &str 不拥有数据
    let slice: &str = &greeting[0..7]; // "你好,"
    
    println!("{}", slice);
}
```

### 什么是 String？

`String` 是一个拥有的、可增长的、堆分配的字符串类型。它拥有自己的数据，并且可以被修改。

```rust, editable
fn main() {
    // String 拥有数据
    let mut owned = String::from("你好");
    
    // 可以修改拥有的字符串
    owned.push_str(", 世界!");
    
    println!("{}", owned); // "你好, 世界!"
}
```

### 关键区别

| 特性 | `&str` | `String` |
|---|---|---|
| **所有权** | 借用的 (引用) | 拥有的 |
| **内存** | 栈 (指针 + 长度) | 堆分配 |
| **可变性** | 不可变 | 可以是可变的 |
| **大小** | 固定大小 | 可增长 |
| **成本** | 传递开销小 | 复制开销大 |
| **生命周期** | 有生命周期参数 | 不需要生命周期 |

### 何时使用

**使用 `&str` 当:**
- 你只需要读取字符串
- 在函数中接受字符串参数
- 使用字符串字面量
- 你想借用字符串的一部分

**使用 `String` 当:**
- 你需要拥有字符串数据
- 构建或修改字符串
- 从函数返回字符串
- 在结构体或集合中存储字符串

### 常见转换

```rust, editable
fn main() {
    // &str 到 String
    let str_slice: &str = "你好";
    let owned: String = str_slice.to_string();
    let owned2: String = String::from(str_slice);
    let owned3: String = str_slice.to_owned();
    
    // String 到 &str (通过 Deref 强制转换自动进行)
    let owned = String::from("你好");
    let borrowed: &str = &owned;
    let borrowed2: &str = owned.as_str();
    
    println!("{} {}", borrowed, borrowed2);
}
```

### 函数参数

总是优先为函数参数使用 `&str` - 它更灵活:

```rust, editable
// 好: 同时接受 &str 和 String
fn print_text(text: &str) {
    println!("{}", text);
}

// 덜 유연함: String만 받음
fn print_text_owned(text: String) {
    println!("{}", text);
}

fn main() {
    let owned = String::from("拥有的");
    let borrowed = "借用的";
    
    // 对两种类型都有效
    print_text(&owned);     // String 自动解引用为 &str
    print_text(borrowed);   // &str 直接工作
    
    // 只对 String 有效
    print_text_owned(owned.clone());
    // print_text_owned(borrowed); // 错误: 期望 String, 发现 &str
}
```

### 字符串切片

你可以从 `String` 创建 `&str` 切片:

```rust, editable
fn main() {
    let s = String::from("你好, 世界!");
    
    // 创建切片
    let hello: &str = &s[0..7];
    let world: &str = &s[9..15];
    
    println!("{} {}", hello, world); // "你好, 世界"
    
    // 完整切片
    let full: &str = &s[..];
}
```

### 字符串字面量

你代码中的字符串字面量是 `&'static str` - 它们在整个程序运行期间都存在:

```rust,ignore
fn main() {
    // 这是一个 &'static str
    let literal: &'static str = "我永远存在!";
    
    // 字符串字面量被嵌入在二进制文件中
    let another = "也是静态的";
}
```

### 构建字符串

```rust, editable
fn main() {
    // 使用 String::new() 和 push_str
    let mut s = String::new();
    s.push_str("你好");
    s.push(' ');
    s.push_str("世界");
    
    // 使用 format! 宏
    let name = "Alice";
    let greeting = format!("你好, {}!", name);
    
    // 使用 + 运算符 (消耗第一个 String)
    let s1 = String::from("你好, ");
    let s2 = String::from("世界!");
    let s3 = s1 + &s2; // s1 被移动, s2 被借用
    
    // 使用连接
    let mut result = String::from("a");
    result += "b";
    result += "c";
    
    println!("{}", result);
}
```

### 内存布局

```rust, editable
fn main() {
    // &str: 只是一个指针和长度 (栈)
    let str_slice: &str = "你好";
    // 内存: [ptr: 8 字节, len: 8 字节] = 栈上 16 字节
    
    // String: 指针、长度和容量 (栈) + 数据 (堆)
    let string: String = String::from("你好");
    // 内存: [ptr: 8 字节, len: 8 字节, cap: 8 字节] = 栈上 24 字节
    //         + 堆上的实际字符串数据
    
    println!("&str 大小: {} 字节", std::mem::size_of::<&str>());
    println!("String 大小: {} 字节", std::mem::size_of::<String>());
}
```

### 性能考虑

```rust,ignore
fn main() {
    // 高效: 无分配
    let s: &str = "你好";
    
    // 需要分配
    let owned = String::from("你好");
    
    // 克隆 &str 开销小 (只复制指针+长度)
    let s2 = s;
    
    // 克隆 String 开销大 (复制堆数据)
    let owned2 = owned.clone();
}
```

### 常见模式

**从函数返回字符串:**

```rust, editable
// 当返回字符串字面量或切片时返回 &str
fn get_greeting() -> &'static str {
    "你好!"
}

// 当构建新字符串时返回 String
fn get_full_name(first: &str, last: &str) -> String {
    format!("{} {}", first, last)
}

fn main() {
    println!("{}", get_greeting());
    println!("{}", get_full_name("John", "Doe"));
}
```

**在结构体中存储:**

```rust,ignore
// 使用带生命周期注解的 &str
struct BorrowedName<'a> {
    name: &'a str,
}

// 当结构体拥有数据时使用 String
struct OwnedName {
    name: String,
}

fn main() {
    let borrowed = BorrowedName { name: "Alice" };
    let owned = OwnedName { name: String::from("Bob") };
}
```

### 最佳实践

- **函数参数优先使用 `&str`** - 更灵活高效
- **需要所有权时使用 `String`** - 当你需要存储或修改字符串时
- **避免不必要的 `.to_string()` 调用** - 尽可能使用 `&str`
- **使用 `format!`** 进行复杂字符串构建，而不是重复连接
- **记住字符串字面量是 `&str`** - 不需要分配
- **在需要明确时显式使用 `.as_str()`**，但 Deref 强制转换通常有效
- **注意 UTF-8** - 切片必须在字符边界上，否则会 panic
