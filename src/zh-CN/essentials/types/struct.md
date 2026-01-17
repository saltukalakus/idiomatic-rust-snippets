### 什么是结构体（struct）？

在 Rust 中，结构体是自定义数据类型，允许你将相关数据组合在一起。它们类似于面向对象语言中的类，可以通过 `impl` 块定义关联方法。结构体用于创建可用于建模现实世界实体的复杂数据类型。

### 结构体的类型

**经典结构体（Classic Structs）**：

这是最常见的结构体类型，每个字段都有名称和类型，称为具名字段。例如：

```rust, editable
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    println!("Name: {}, Age: {}", person.name, person.age);
}
```

**元组结构体（Tuple Structs）**：

类似于元组，但有名称，可用于创建新类型。元组结构体的字段没有名称。例如：

```rust, editable
struct Color(u8, u8, u8);

fn main() {
    let black = Color(0, 0, 0);

    println!("Black color: ({}, {}, {})", black.0, black.1, black.2);
}
```

**单元结构体（Unit-like Structs）**：

没有任何字段的结构体，可用于泛型或 trait 场景。例如：

```rust, editable
struct Unit;

fn main() {
    let unit = Unit;

    println!("Unit struct created!");
}
```

你可以使用 `impl` 关键字为结构体定义方法和关联函数（参见 `imp.md`）。