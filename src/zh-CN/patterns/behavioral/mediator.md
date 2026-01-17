### 中介者模式（Mediator Pattern）

中介者模式将对象之间复杂的通信集中化。对象不直接相互引用，而是通过中介者进行通信。

**优势**：
- 减少组件之间的耦合
- 将交互逻辑集中在一个地方
- 组件可以独立重用
- 易于理解对象交互

```rust, editable
{{#include mediator/src/main.rs}}
```

**关键点**：
- 示例展示了 `Component1` 和 `Component2` 通过 `ConcreteMediator` 进行通信
- 每个组件持有对中介者的 `Arc<Mutex<ConcreteMediator>>` 引用
- 当 `Component1` 调用 `operation_a()` 时，它通知中介者，中介者然后更新 `Component2`
- 当 `Component2` 调用 `operation_b()` 时，中介者处理它并更新 `Component1`
- 组件永远不会直接引用彼此 - 所有通信都通过中介者流动

**何时使用**：
- 多个对象之间的复杂通信
- 对象紧密耦合且难以重用
- 交互的 UI 组件（对话框、表单）
- 聊天系统、空中交通管制或协调逻辑
