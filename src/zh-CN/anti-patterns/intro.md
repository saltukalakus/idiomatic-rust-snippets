### 非惯用 Rust 用法

受 [Corroded](https://github.com/buyukakyuz/corroded) 启发，本节介绍常见的反模式和非惯用 Rust 代码，以及推荐的惯用替代方案。了解应该避免什么与学习最佳实践同样重要。

理解这些反模式有助于编写更惯用、更高效、更易维护的 Rust 代码，通过识别和避免常见陷阱。

1- [所有权与内存](./ownership-memory/intro.md)：与所有权、借用、生命周期和内存管理相关的反模式。

2- [错误处理](./error-handling/intro.md)：使用 `Result`、`Option` 和 panic 场景时的常见错误。

3- [代码结构](./code-structure/intro.md)：代码组织、控制流和利用语言特性时的反模式。

4- [类型系统](./type-system/intro.md)：Rust 类型系统的误用，包括字符串处理和类型转换。