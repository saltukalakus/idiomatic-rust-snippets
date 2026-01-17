### 闭包（Closures）

闭包是可以捕获其环境的匿名函数，类似于其他语言中的 lambda。Rust 中使用 `|` 语法定义闭包。

### 基本语法

```rust, editable
let add = |a, b| a + b;
println!("和: {}", add(2, 3)); // Output: 和: 5
```

### 捕获环境

闭包可以从其外层作用域捕获变量。

```rust, editable
let x = 10;
let print_x = || println!("x: {}", x);
print_x(); // Output: x: 10
```

### 可变捕获

闭包也可以以可变方式捕获变量。

```rust, editable
let mut x = 10;
{
    let mut add_to_x = || x += 5;
    add_to_x();
}
println!("x: {}", x); // Output: x: 15
```

### 移动捕获

闭包可以使用 `move` 关键字获取所捕获变量的所有权。

```rust, editable
let x = vec![1, 2, 3];
let print_x = move || println!("x: {:?}", x);
print_x(); // Output: x: [1, 2, 3]
// x 在此处不再可访问
```

闭包是 Rust 中一个强大功能，使代码更简洁且富有表达力。
