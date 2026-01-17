### 标量类型

标量类型表示单个值。Rust 有四种主要的标量类型：

**整数（Integers）**：整数，包含有符号和无符号。
**浮点数（Floating-Point Numbers）**：带小数点的数字。
**布尔值（Booleans）**：真或假。
**字符（Characters）**：单个 Unicode 字符。

```rust, editable
fn main() {
    let x: i32 = 42; // Integer
    let y: f64 = 3.14; // Floating-point number
    let is_active: bool = true; // Boolean
    let letter: char = 'A'; // Character

    println!("x: {}, y: {}, is_active: {}, letter: {}", x, y, is_active, letter);
}
```
