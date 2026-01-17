### 错用 Deref 强制转换

实现 `Deref` 来模拟继承关系是对该 trait 的误用。`Deref` 设计用于智能指针（如 `Box<T>`、`Rc<T>`）以透明访问内部类型。将其用于“is-a”关系会产生令人困惑的 API，类型会意外发生强制转换，方法解析变得不可预测。

`Deref` 仅应在类型表现得像智能指针时实现。对于组合，应使用显式字段和方法。

```rust, editable
use std::ops::Deref;

struct User {
    name: String,
    email: String,
}

struct Admin {
    user: User,
    permissions: Vec<String>,
}

impl Deref for Admin {
    type Target = User;
    
    fn deref(&self) -> &Self::Target {
        &self.user
    }
}

impl User {
    fn display(&self) {
        println!("{} <{}>", self.name, self.email);
    }
}

fn main() {
    let admin = Admin {
        user: User {
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        },
        permissions: vec!["read".to_string(), "write".to_string()],
    };
    
    admin.display(); // 通过 Deref 能工作，但容易混淆
    println!("Name: {}", admin.name); // 也能通过 Deref 强制转换访问
}
```

`Admin` 对 `User` 进行 deref，创建了隐式的“is-a”关系，但 Rust 并不鼓励这种做法。这会让代码使用者困惑，因为 `Admin` 并不是指向 `User` 的智能指针。像 `admin.display()` 这样的调用能通过自动 deref 工作，但并不直观，读者必须知道 `Admin` 会自动 deref 到 `User`。这种模式模仿了继承，而 Rust 有意不支持继承。

下面的示例使用显式组合来改进。

```rust, editable
struct User {
    name: String,
    email: String,
}

struct Admin {
    user: User,
    permissions: Vec<String>,
}

impl User {
    fn display(&self) {
        println!("{} <{}>", self.name, self.email);
    }
}

impl Admin {
    fn user(&self) -> &User {
        &self.user
    }
    
    fn display(&self) {
        self.user.display();
        println!("Permissions: {:?}", self.permissions);
    }
}

fn main() {
    let admin = Admin {
        user: User {
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        },
        permissions: vec!["read".to_string(), "write".to_string()],
    };
    
    admin.display(); // 清楚地调用 Admin 的 display
    println!("Name: {}", admin.user().name); // 明确访问 user 的数据
}
```

**关键改进**：
- 不滥用 `Deref` —— 组合更显式
- `Admin` 提供 `user()` 访问器方法以按需访问内部 `User`
- `Admin::display()` 可以根据需要扩展或包装 `User::display()`
- 清晰知道调用的是哪个类型的方法 —— 无隐藏的强制转换
- 遵循 Rust 的惯用法 —— 偏向组合而非 `Deref` 技巧

