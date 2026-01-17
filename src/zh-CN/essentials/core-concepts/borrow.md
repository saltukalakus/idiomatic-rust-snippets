### 借用

借用允许你在不取得值所有权的情况下引用该值。Rust 中有两种借用方式：

**不可变借用（&T）**：

你可以对一个值拥有多个不可变引用。

```rust, editable
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // 不可变借用 s1

    println!("The length of '{}' is {}.", s1, len); // s1 在此仍然有效
}

fn calculate_length(s: &String) -> usize {
    s.len() // 使用被借用的值
}
```

`calculate_length` 对 `s1` 做了不可变借用。该函数可以读取值但不能修改它。函数调用后，`s1` 仍然有效。

**可变借用（&mut T）**：

在同一时间，你只能对一个值拥有一个可变引用。

```rust, editable
fn main() {
    let mut s = String::from("hello");
    change(&mut s); // 可变借用 s

    println!("{}", s); // s 在此仍然有效并被修改
}

fn change(some_string: &mut String) {
    some_string.push_str(", world"); // 修改被借用的值
}
```

`change` 函数对 `s` 做了可变借用。该函数可以修改值。函数调用后，`s` 仍然有效并已被修改。
