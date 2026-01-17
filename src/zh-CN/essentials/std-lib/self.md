### **self** 是 Rust 中的一个特殊关键字吗？

**self** 是 Rust 中的一个特殊关键字。它用于在其关联方法中引用结构体或枚举的实例。它类似于其他面向对象语言（如 Java 或 C++）中的 `this`。

### self 的用法

**方法定义**：在为结构体或枚举定义方法时，`self` 用于引用调用该方法的实例。
**方法参数**：`self` 可以在方法定义中用作参数，以指示该方法获取实例的所有权、不可变借用或可变借用。

### 不可变借用 (`&self`)

当一个方法以 `&self` 作为参数时，意味着该方法不可变地借用实例。

```rust, editable
pub struct Person {
    name: String,
}

impl Person {
    // 不可变地借用实例的方法
    fn greet(&self) {
        println!("你好，我的名字是 {}！", self.name);
    }
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
    };
    person.greet(); // 调用 greet 方法
}
```

### 可变借用 (`&mut self`)

当一个方法以 `&mut self` 作为参数时，意味着该方法可变地借用实例。

```rust, editable
pub struct Counter {
    count: i32,
}

impl Counter {
    // 可变地借用实例的方法
    fn increment(&mut self) {
        self.count += 1;
    }
}

fn main() {
    let mut counter = Counter { count: 0 };
    counter.increment(); // 调用 increment 方法
    println!("计数: {}", counter.count);
    counter.increment(); // 调用 increment 方法
    println!("计数: {}", counter.count);
}
```

### 所有权 (`self`)

当一个方法以 `self` 作为参数时，意味着该方法获取实例的所有权。

```rust, editable
pub struct Person {
    name: String,
}

impl Person {
    // 获取实例所有权的方法
    fn into_name(self) -> String {
        self.name
    }
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
    };
    let name = person.into_name(); // 调用 into_name 方法，获取所有权
    println!("名字: {}", name);
}
```
