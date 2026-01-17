### 如何从标准库调用函数：

```rust, editable
{{#include modules/src/stdlib/main.rs}}
```

### 如何从 Crates.io 上的库调用函数：

对于外部库，可以在 Cargo.toml 文件中指定外部模块。

```rust,noplaypen
{{#include modules/src/external-lib/main.rs}}
```

Cargo.toml 文件内容

```toml
{{#include ./modules/src/external-lib/Cargo.toml}}
```

### Rust 2015 版或更早版本的遗留解决方案

在 Rust 文件中，可以调用 `extern crate PACKAGE_NAME` 来使用外部库。这是在使用 [Rust Playground](https://play.rust-lang.org/) 时的便捷解决方案，如本例所示。否则，执行文件时会出错。

```rust, editable
{{#include modules/src/rand/main.rs}}
```
