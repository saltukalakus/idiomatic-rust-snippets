### Crates（包）

在 Rust 中，crate 是代码发布与分发的最小单元。crate 可以生成可执行程序或库。crate 分为两类：

1. **二进制 crate（Binary Crates）**：编译生成可执行程序。
2. **库 crate（Library Crates）**：一组可被其它程序重用的代码。

### 创建二进制 crate

可使用 `cargo new` 命令创建二进制 crate：

```sh
cargo new my_binary_crate
```

该命令会创建如下目录结构：

```
my_binary_crate/
├── Cargo.toml
└── src
    └── main.rs
```

`main.rs` 是二进制 crate 的入口点。

```rust, editable
fn main() {
    println!("Hello, world!");
}
```

### 创建库 crate

使用 `--lib` 标志创建库 crate：

```sh
cargo new my_library_crate --lib
```

目录结构为：

```
my_library_crate/
├── Cargo.toml
└── src
    └── lib.rs
```

`lib.rs` 是库 crate 的入口点。

```rust,noplaypen
pub fn hello() {
    println!("Hello, library!");
}
```

### 使用 Crates

要在项目中使用外部 crate，需要将其添加到 `Cargo.toml`：例如使用 `rand`：

```toml
[dependencies]
rand = "0.8"
```

然后在代码中使用该 crate：

```rust,noplaypen
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let n: u32 = rng.gen();
    println!("Random number: {}", n);
}
```

理解如何创建和使用 crate 可以帮助你充分利用 Rust 强大的模块系统。

