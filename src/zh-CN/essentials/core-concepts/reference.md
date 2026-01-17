### 引用

引用允许你引用某个值而无需取得其所有权。引用默认是不可变的，意味着你不能修改它们指向的值。但是，你可以创建可变引用来修改值。

### 不可变引用

不可变引用使用 `&` 符号创建。

```rust, editable
fn main() {
    let x = 5;
    let y = &x; // y 是 x 的一个不可变引用

    println!("x: {}, y: {}", x, y);
}
```

`y` 是 `x` 的一个不可变引用。你可以通过 `y` 读取 `x` 的值，但不能修改它。

### 可变引用

可变引用使用 `&mut` 符号创建。

```rust, editable
fn main() {
    let mut x = 5;
    let y = &mut x; // y 是 x 的一个可变引用

    *y += 1; // 通过 y 修改 x 的值

    println!("x: {}", x);
}
```

`y` 是 `x` 的一个可变引用。你可以通过 `y` 修改 `x` 的值。

### 引用规则

1. 在任何给定时间，你要么只能有一个可变引用，要么只能有任意数量的不可变引用。
2. 引用必须始终有效。

这些规则确保了内存安全，并在编译时防止数据竞争。

### 函数示例

你也可以将引用传递给函数：

```rust, editable
fn main() {
    let x = 5;
    print_value(&x); // 将不可变引用传递给函数

    let mut y = 10;
    increment(&mut y); // 将可变引用传递给函数

    println!("y: {}", y);
}

fn print_value(val: &i32) {
    println!("值: {}", val);
}

fn increment(val: &mut i32) {
    *val += 1;
}
```

`print_value` 接受一个不可变引用，而 `increment` 接受一个可变引用。
