### 抽象工厂模式

抽象工厂提供一个接口，用于创建相关对象族而不指定其具体类型。这确保创建的对象彼此兼容。

**优势**：
- 创建协同工作的相关对象族
- 将具体类与客户端代码隔离
- 轻松交换整个产品族
- 确保相关产品之间的一致性

```rust, editable
{{#include abstract-method/src/main.rs}}
```

**关键点**：
- 示例为家具产品定义了 `Chair` 和 `Sofa` trait
- `FurnitureFactory` trait 声明 `create_chair()` 和 `create_sofa()` 方法
- `ModernFurnitureFactory` 创建现代风格的椅子和沙发；`VictorianFurnitureFactory` 创建维多利亚风格的
- 客户端接收工厂并调用方法获取匹配的家具，而无需知道具体类型
- 来自一个工厂的所有家具共享相同风格，确保视觉一致性

**何时使用**：
- 需要相关对象族时
- 确保产品协同工作时（相同主题/风格）
- 想在运行时切换产品族时
- 用于 UI 主题、数据库驱动程序或平台特定组件