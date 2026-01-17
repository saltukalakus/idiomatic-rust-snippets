### 什么是 `trait`？

在 Rust 中，`trait` 是以抽象方式定义共享行为的一种方式，类似于其他语言中的接口。`trait` 定义了一组类型必须实现的方法。`trait` 用于定义共享行为并启用多态。

### `trait` 的用途

**定义共享行为**：`trait` 允许你定义可以在不同类型之间共享的方法。<br/>
**启用多态**：`trait` 支持多态，使你可以以通用方式编写可操作于不同类型的代码。<br/>
**抽象类型**：`trait` 使你能够对类型进行抽象，从而编写更灵活和可重用的代码。<br/>

下例中定义了一个名为 `Greet` 的 `trait`，包含一个方法 `greet`。随后为 `Person` 和 `Dog` 两个结构体实现了该 `trait`。

```rust, editable
// 定义一个名为 `Greet` 的 trait
pub trait Greet {
    fn greet(&self);
}

// 定义一个名为 `Person` 的结构体
pub struct Person {
    name: String,
}

// 为 `Person` 结构体实现 `Greet` trait
impl Greet for Person {
    fn greet(&self) {
        println!("你好，我的名字是 {}！", self.name);
    }
}

// 定义一个名为 `Dog` 的结构体
pub struct Dog {
    name: String,
}

// 为 `Dog` 结构体实现 `Greet` trait
impl Greet for Dog {
    fn greet(&self) {
        println!("汪！我的名字是 {}！", self.name);
    }
}


fn main() {
    // 创建 `Person` 和 `Dog` 的实例
    let person = Person {
        name: String::from("Alice"),
    };
    let dog = Dog {
        name: String::from("Buddy"),
    };
    dog.greet();
    person.greet();
}
```
