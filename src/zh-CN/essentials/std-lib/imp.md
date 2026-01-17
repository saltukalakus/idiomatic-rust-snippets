### **imp** 关键字的用途是什么？

Rust 中的 **impl** 关键字用于为结构体、枚举和 [traits](./trait.md) 定义实现。它允许您将函数和方法与类型关联，并为类型实现 trait。

### impl 的用法

**为结构体或枚举实现方法**：

您可以定义与结构体或枚举关联的方法。

`pub fn new(name: String, age: u8) -> Self` 是一个关联函数（通常用作构造函数），用于创建 `Person` 的新实例。`pub fn greet(&self)` 和 `pub fn have_birthday(&mut self)` 是与 `Person` 结构体关联的方法。`greet` 方法不可变地借用实例，而 `have_birthday` 方法可变地借用它。

```rust, editable
pub struct Person {
    name: String,
    age: u8,
}

impl Person {
    // 关联函数（构造函数）
    pub fn new(name: String, age: u8) -> Self {
        Person { name, age }
    }

    // 不可变地借用实例的方法
    pub fn greet(&self) {
        println!("你好，我叫 {}，我 {} 岁了。", self.name, self.age);
    }

    // 可变地借用实例的方法
    pub fn have_birthday(&mut self) {
        self.age += 1;
    }
}

fn main() {
    let mut person = Person::new(String::from("Alice"), 30);
    person.greet();
    person.have_birthday();
    person.greet();
}
```

**为结构体或枚举实现 Trait**：

您可以为结构体或枚举实现 trait，定义 trait 所需的行为。

`Greet` trait 定义了一个 `greet` 方法。`impl Greet for Dog` 块为 `Dog` 结构体实现了 `Greet` trait，提供了 trait 所需的行为。

```rust, editable
pub trait Greet {
    fn greet(&self);
}

pub struct Dog {
    name: String,
}

impl Dog {
    pub fn new(name: String) -> Self {
        Dog { name }
    }
}

impl Greet for Dog {
    fn greet(&self) {
        println!("汪！我叫 {}！", self.name);
    }
}

fn main() {
    let dog = Dog::new(String::from("Buddy"));
    dog.greet();
}
```
