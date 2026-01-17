### 流程控制

Rust 提供了多种流程控制选项，包括 `if` 语句、`while` 循环等。下面是一些简要示例：

### `if` 语句

`if` 语句允许你根据条件执行代码。

```rust, editable
let number = 5;

if number < 10 {
    println!("数字小于 10");
} else {
    println!("数字大于等于 10");
}
```

### `while` 循环

`while` 循环在条件为真时重复执行代码。

```rust, editable
let mut count = 0;

while count < 5 {
    println!("计数：{}", count);
    count += 1;
}
```

### `for` 循环

`for` 循环允许你遍历一个范围或集合。

```rust, editable
for number in 1..5 {
    println!("数字是：{}", number);
}
```

### `match` 语句

`match` 语句允许你将一个值与一系列模式进行比较，并根据匹配到的模式执行代码。

```rust, editable
let number = 3;

match number {
    1 => println!("一"),
    2 => println!("二"),
    3 => println!("三"),
    _ => println!("其他"),
}
```

以上是 Rust 中一些基本的流程控制结构，每个结构都能帮助你有效管理程序的执行流程。