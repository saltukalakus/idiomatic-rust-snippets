### 什么是泛型（Generics）？

Rust 中的泛型允许你编写灵活且可重用的代码，使你可以定义函数、`struct`、`enum` 和 `trait`，这些可以在不牺牲性能的情况下作用于多种类型。泛型是帮助你编写更抽象且类型安全代码的强大特性。

### 泛型的关键概念

泛型函数可以在函数调用时指定不同类型。

在示例中，函数 `larger` 使用泛型类型参数 `T`。类型 `T` 必须实现 `PartialOrd`（用于比较）和 `Copy`（以便按值返回）。该函数比较两个值并返回较大的一个。

```rust, editable
fn larger<T: PartialOrd + Copy>(x: T, y: T) -> T {
    if x > y {
        x
    } else {
        y
    }
}

fn main() {
    let a = 10;
    let b = 20;
    let result = larger(a, b);
    println!("The larger value is: {}", result);  // Prints: 20

    let c = 5.5;
    let d = 2.3;
    let result = larger(c, d);
    println!("The larger value is: {}", result);  // Prints: 5.5
}
```

结构体可以在实例化时持有不同类型的数据。

下面 `Point` 结构体被实现为同时适用于整数和浮点类型。

```rust, editable
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };

    println!("Integer Point: ({}, {})", integer_point.x, integer_point.y);
    println!("Float Point: ({}, {})", float_point.x, float_point.y);
}
```

枚举可以在实例化时持有不同类型的变体。

下面是一个自定义泛型枚举，类似标准库中的 `Option<T>`：

```rust, editable
#[derive(Debug)]
enum MyOption<T> {
    Some(T),
    None,
}

fn main() {
    let some_number = MyOption::Some(5);
    let some_string = MyOption::Some("a string");
    let no_value: MyOption<i32> = MyOption::None;

    println!("Some number: {:?}", some_number);
    println!("Some string: {:?}", some_string);
    println!("No value: {:?}", no_value);
}
```

**注意**：Rust 标准库已提供结构相同的 `Option<T>`，该类型在每个 Rust 程序中都可用。