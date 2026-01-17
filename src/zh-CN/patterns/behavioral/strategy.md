### 策略（Strategy）模式

策略模式将一组可互换的算法定义为一个家族。每个算法封装为独立的类型，允许在运行时进行替换。

**优点**:
- 在运行时切换算法
- 消除用于选择算法的条件逻辑
- 每个算法可独立测试
- 添加新策略无需修改现有代码

```rust, editable
{{#include strategy/src/main.rs}}
```

**要点**:
- 示例定义了带有 `execute(&self, data: &str)` 方法的 `Strategy` trait
- `ConcreteStrategyA` 和 `ConcreteStrategyB` 实现了不同的执行行为
- `Context` 持有 `Box<dyn Strategy>`，并通过 `execute_strategy()` 将调用委托给策略
- 在 `main()` 中，`Context` 先使用策略 A 执行，然后通过 `set_strategy()` 切换到策略 B
- 若希望使用编译时分发（零成本），可使用泛型：`Context<S: Strategy>` 代替 `Box<dyn Strategy>`

**何时使用**:
- 同一任务存在多种算法实现时
- 需要根据配置在运行时选择算法时
- 希望用多态替代大量条件逻辑时
- 排序、压缩、加密等有多种实现时
