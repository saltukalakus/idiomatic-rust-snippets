### 如何在 Rust 项目中使用不稳定特性

在 Rust 中，`#!` 语法用于适用于整个 crate 或模块的属性，称为"内部属性"。它们通常放置在文件顶部，用于配置 Rust 编译器对该文件或 crate 的各种行为。

`#![feature(...)]`：此属性允许在代码中使用来自 nightly Rust 编译器的不稳定特性。它必须放在文件顶部。

例如，Rust 中的 `#![feature(test)]` 属性用于启用 test crate，其中包含尚未稳定的基准测试和测试实用程序。

```rust,noplaypen
#![feature(test)]
extern crate bcrypt;
extern crate test;

use bcrypt::{hash, DEFAULT_COST};

#[bench]
fn bench_cost_4(b: &mut test::Bencher) {
    b.iter(|| hash("hunter2", 4));
}
```
