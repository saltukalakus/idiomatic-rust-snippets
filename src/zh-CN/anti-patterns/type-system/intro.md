### 类型系统反模式

Rust 的类型系统旨在在编译时捕获错误并清晰地表达意图。与类型系统对抗或误用它，会导致代码更难理解和维护。

本节覆盖与 Rust 类型系统相关的常见反模式：

1- [String 与 str 的混淆](./string-vs-str.md)：在 `&str` 就足够的地方使用 `String`，导致不必要的分配并使 API 可变性降低。

2- [与类型系统对抗](./fighting-type-system.md)：使用过多的类型转换或 unsafe 代码来绕过类型安全，而不是正确利用 Rust 的类型系统。

3- [误用 Deref 协变](./deref-abuse.md)：为不应视为智能指针的类型过度实现 `Deref`，违反最小惊讶原则并制造令人困惑的 API。

4- [字符串式编码（Stringly Typed）](./stringly-typed.md)：将字符串用于所有场景，而不是使用适当的类型或枚举以获得更好的类型安全和编译时保证。
