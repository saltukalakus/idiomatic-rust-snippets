### 策略模式

策略模式定义了一系列可互换的算法。每个算法都封装在自己的类型中，允许在运行时交换它们。

**优势**：
- 在运行时交换算法
- 消除算法选择的条件逻辑
- 每个算法都可以独立测试
- 易于添加新策略而无需修改现有代码

```rust, editable
{{#include ../../../patterns/behavioral/strategy/src/main.rs}}
```

**关键点**：
- 示例定义了带有 `execute(&self, data: &str)` 方法的 `Strategy` trait
- `ConcreteStrategyA` 和 `ConcreteStrategyB` 实现不同的执行行为
- `Context` 持有 `Box<dyn Strategy>` 并通过 `execute_strategy()` 委托给它
- 在 `main()` 中，上下文以策略 A 开始执行，然后使用 `set_strategy()` 切换到策略 B
- 对于编译时分派（零成本），使用泛型：`Context<S: Strategy>` 而不是 `Box<dyn Strategy>`

**何时使用**：
- 同一任务有多个算法
- 基于配置的运行时算法选择
- 用多态替换条件逻辑
- 排序、压缩、加密等有多种实现的场景