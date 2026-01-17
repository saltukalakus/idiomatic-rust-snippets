### 函数

函数是 Rust 中的基本构建块，允许你将代码封装为可重用的片段。函数使用 `fn` 关键字定义，后跟函数名、参数和函数体。

### 定义函数

```rust, editable
fn main() {
    println!("你好，世界！");
}
```

`main` 是特殊函数，作为 Rust 程序的入口点。

### 函数参数

函数可以接收参数，参数在函数名后面的括号中指定。

```rust, editable
fn greet(name: &str) {
    println!("你好，{}！", name);
}

fn main() {
    greet("Alice");
    greet("Bob");
}
```

- `greet` 函数接收一个类型为 `&str` 的参数 `name`。

### 返回值

函数也可以返回值。返回类型在箭头（`->`）之后指定。

```rust, editable
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let sum = add(5, 3);
    println!("和: {}", sum);
}
```

- `add` 函数接收两个 `i32` 类型参数并返回它们的和，返回类型为 `i32`。

### 提前返回

可以使用 `return` 关键字提前从函数返回值：

```rust, editable
fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    }
    false
}

fn main() {
    let number = 4;
    if is_even(number) {
        println!("{} is even", number);
    } else {
        println!("{} is odd", number);
    }
}
```

- `is_even` 函数在数字为偶数时返回 `true`，否则返回 `false`。