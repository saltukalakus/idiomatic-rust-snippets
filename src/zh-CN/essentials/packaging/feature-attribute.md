```markdown
### 特性属性（feature attribute）

Rust 的 `#[cfg(feature = "foo")]` 与 `features` 字段允许基于功能开关编译条件代码，常用于可选依赖或平台差异处理。

（翻译说明：示例展示如何在 `Cargo.toml` 中声明特性并在代码中使用 `cfg` 条件编译。）
```
