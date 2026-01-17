### Rust 中的模块

模块是一种将代码组织到不同命名空间的方式。它们有助于管理作用域和私有性，使代码更加模块化且易于维护。

### 定义模块

你可以使用 `mod` 关键字定义一个模块。下面是一个基本示例：

```rust,editable
mod my_module {
    pub fn say_hello() {
        println!("你好，来自 my_module！");
    }
}

fn main() {
    my_module::say_hello();
}
```

- `mod my_module` 定义了一个名为 `my_module` 的模块。
- `pub fn say_hello()` 在模块中定义了一个公共函数。
- `my_module::say_hello()` 从 main 函数中调用该函数。

### 嵌套模块

模块可以嵌套在其他模块中：

```rust,editable
mod outer_module {
    pub mod inner_module {
        pub fn say_hello() {
            println!("你好，来自 inner_module！");
        }
    }
}

fn main() {
    outer_module::inner_module::say_hello();
}
```

- `pub mod inner_module` 在 `outer_module` 中定义了一个嵌套模块。
- `outer_module::inner_module::say_hello()` 从 main 函数中调用该函数。

### 模块文件

模块也可以在单独的文件中定义。例如，你可以创建如下的文件结构：

```
src/
├── main.rs
└── my_module.rs
```

在 `main.rs` 中：

```rust,noplaypen
mod my_module;

fn main() {
    my_module::say_hello();
}
```

在 `my_module.rs` 中：

```rust,noplaypen
pub fn say_hello() {
    println!("你好，来自 my_module！");
}
```

这种结构有助于保持代码库的组织性和可管理性。
