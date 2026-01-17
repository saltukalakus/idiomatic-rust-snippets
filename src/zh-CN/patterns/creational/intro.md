# 创建型模式（Creational Patterns）

创建型模式关注对象的创建过程，提供不同的方式来实例化对象，使代码更解耦、更易测试。常见模式包括 `builder`、`factory-method`、`singleton`、`prototype` 等。

本节展示在 Rust 中实现这些模式的惯用方法与注意事项，通常更倾向于使用 `trait`、关联函数与所有权语义而不是经典 OOP 的实现细节。