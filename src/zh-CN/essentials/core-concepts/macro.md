### Rust 中的宏

Rust 中的宏是一种编写代码来生成其他代码的方式，这被称为元编程。它们用于减少代码重复并提供更具表现力的语法。

### 宏的类型

1.  **声明式宏 (macro_rules!)**
2.  **过程式宏**

### 声明式宏

声明式宏使用 `macro_rules!` 关键字定义。它们允许您匹配模式并根据这些模式生成代码。

```rust,ignore
macro_rules! say_hello {
    () => {
        println!("你好，世界！");
    };
}

fn main() {
    say_hello!();
}
```

### 过程式宏

过程式宏更加灵活和强大。它们使用函数定义，可用于创建类属性宏和类函数宏。

#### 类函数宏

类函数宏看起来像函数调用，但它们操作的是作为参数传递给它们的代码。它们使用 `#[proc_macro]` 属性定义，可用于根据提供的输入生成代码。

要创建过程式宏，您需要创建一个带有 `proc-macro` 属性的新 crate：

```rust,ignore
// 在您的 Cargo.toml 中
[lib]
proc-macro = true
```

然后，在您的 crate 中定义宏：

```rust,ignore
extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn my_function_like_macro(input: TokenStream) -> TokenStream {
    // 在这里实现您的宏
    input
}
```

并在您的主 crate 中使用它：

```rust,ignore
use my_function_like_macro::my_function_like_macro;

my_function_like_macro! {
    // 您的代码在这里
}
```

`my_function_like_macro` 是一个类函数宏，可以使用 `!` 语法调用。该宏处理输入的 token 流并生成相应的代码。

#### 类属性宏

类属性宏用于创建可应用于函数、结构体或模块等项的自定义属性。它们的定义方式与过程式宏类似。

首先，创建一个带有 `proc-macro` 属性的新 crate：

```rust,ignore
// 在您的 Cargo.toml 中
[lib]
proc-macro = true
```

然后，在您的 crate 中定义类属性宏：

```rust,ignore
extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn my_attribute(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // 在这里实现您的宏
    item
}
```

并在您的主 crate 中使用它：

```rust,ignore
use my_attribute::my_attribute;

#[my_attribute]
fn my_function() {
    println!("这个函数有一个自定义属性！");
}

fn main() {
    my_function();
}
```

`my_attribute` 是一个可以应用于函数的类属性宏。当调用 `my_function` 时，它会向控制台打印一条消息。

### 用法

宏使用 `!` 语法调用。例如：

```rust,ignore
println!("你好，{}！", "世界");
```

`println!` 是一个向控制台打印格式化文本的宏。
