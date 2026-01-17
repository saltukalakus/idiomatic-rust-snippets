### 工厂方法模式（Factory Method Pattern）

工厂方法提供了创建对象的接口，允许实现决定实例化哪个具体类型。这将对象创建与使用对象的代码解耦。

**优势**：
- 将对象创建与使用解耦
- 易于添加新类型而无需修改现有代码
- 促进依赖注入和可测试性
- 使用 trait 实现多态行为

```rust, editable
{{#include factory-method/src/main.rs}}
```

**关键点**：
- 示例定义了带有 `draw()` 方法的 `Shape` trait，由 `Circle` 和 `Square` 实现
- `ShapeFactory` trait 声明了 `create_shape()`，返回 `Box<dyn Shape>`
- `CircleFactory` 创建圆形，`SquareFactory` 创建正方形 - 每个都返回适当的类型
- 在 `main()` 中，工厂创建形状而客户端不知道具体类型
- 要添加新形状（例如 Triangle），只需实现 `Shape` trait 并创建 `TriangleFactory`

**何时使用**：
- 当要创建的确切类型直到运行时才知道时
- 将对象创建与业务逻辑解耦
- 当您想在不更改现有代码的情况下扩展对象创建时
- 为了可测试性（注入模拟工厂）