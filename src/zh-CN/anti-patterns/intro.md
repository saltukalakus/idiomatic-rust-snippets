# 反模式（Anti-Patterns）

本章讨论在 Rust 代码中常见的反模式——那些看起来可行但会导致错误、性能下降或不易维护的做法。目标是识别、解释为什么它们是反模式，并提供更好的替代方案。

- **范围**：语义错误、性能陷阱、不安全的所有权/借用模式、以及不 idiomatic 的代码结构。
- **目标读者**：已有 Rust 基础，希望编写更安全、更高效、更可维护代码的开发者。

接下来的小节按主题组织：`code-structure`、`error-handling`、`ownership-memory`、`type-system`。