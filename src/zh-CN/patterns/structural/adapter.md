### 适配器模式（Adapter Pattern）

适配器模式通过用新接口包装现有接口，使不兼容的接口能够协同工作。这对于集成第三方库或遗留代码非常有用。

**优势**：
- 集成不兼容的接口而无需修改它们
- 使用不同接口重用现有功能
- 将客户端代码与特定实现解耦
- 包装外部类型以实现本地 trait

```rust, editable
{{#include adapter/src/main.rs}}
```

**关键点**：
- 示例定义了带有 `request()` 方法的 `Target` trait
- `Adaptee` 有不同名称/签名的 `specific_request()` 方法
- `Adapter` 包装 `Adaptee` 并实现 `Target` trait
- 在 `request()` 实现中，适配器调用 `self.adaptee.specific_request()` 进行委托
- `main()` 中的客户端代码使用 `Target` trait，不知道 `Adaptee` 的实现细节

**何时使用**：
- 集成具有不兼容 API 的第三方库
- 使遗留代码与新接口协同工作
- 在外部类型上实现 trait（孤儿规则的解决方法）
- 为多个类型创建统一的接口