```markdown
### 错误处理反模式

正确的错误处理对于健壮的 Rust 应用至关重要。Rust 的 `Result` 和 `Option` 类型为优雅地处理错误提供了强大的工具，但开发者有时会走捷径，导致脆弱的代码。

本节介绍错误处理中的常见反模式：

1- [滥用 unwrap（Unwrap Abuse）](./unwrap-abuse.md)：在任何地方使用 `.unwrap()` 和 `.expect()`，而不是使用 `Result` 或 `Option` 做出正确处理。

2- [库中 panic（Panic in Libraries）](./panic-in-libraries.md)：在库代码中使用 `panic!` 而不是返回 `Result`，这会阻止库的使用者优雅地处理错误。

```