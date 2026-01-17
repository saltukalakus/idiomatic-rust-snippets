### Rust 中的生命周期

在 Rust 中，生命周期是一种确保引用在需要时有效且不会超过其有效期的机制。生命周期可以防止悬垂引用，从而避免未定义行为。

```rust, editable
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("长字符串很长");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("最长的字符串是 {}", result);
}
```

函数 `longest` 接受两个具有相同生命周期 `'a` 的字符串切片，并返回一个具有相同生命周期 `'a` 的字符串切片。这确保了返回的引用在两个输入引用都有效的情况下是有效的。

### 关键点

- 生命周期用撇号后跟一个名称来表示，如 `'a`。
- 它们帮助 Rust 编译器确保引用不会比它们指向的数据活得更久。
- 在许多情况下，编译器会推断生命周期，但有时需要显式地进行注解。

更多详细信息，请参阅 [Rust 官方文档](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)。
