### 代理模式（Proxy Pattern）

代理模式为另一个对象提供代理或占位符，控制对它的访问。代理具有与真实对象相同的接口。

**优势**：
- 控制对真实对象的访问
- 可以添加功能（日志、缓存、延迟加载）
- 真实对象可以是远程的、创建成本高或需要保护
- 对客户端透明 - 与真实对象具有相同的接口

```rust, editable
{{#include proxy/src/main.rs}}
```

**关键点**：
- 示例定义了带有 `request()` 方法的 `Subject` trait
- `RealSubject` 实现实际的业务逻辑
- `Proxy` 包装 `RealSubject` 并实现相同的 `Subject` trait
- 在 `Proxy::request()` 中，代理在调用 `real_subject.request()` 之前/之后添加日志
- 客户端代码使用 `Subject` trait - 可以交换使用代理或真实主体

**何时使用**：
- 昂贵对象的延迟初始化
- 访问控制或身份验证
- 记录、缓存或监控对象使用
- 远程对象（网络代理）、智能引用