### 建造者（Builder）模式

`Builder` 模式用于逐步构建复杂对象，提供链式配置接口。在 Rust 中常通过带有可选字段的结构体和 `impl` 中的构造器链来实现。

```rust, editable
struct Config { host: String, port: u16 }

impl Config {
    fn builder() -> ConfigBuilder { ConfigBuilder::default() }
}

#[derive(Default)]
struct ConfigBuilder { host: Option<String>, port: Option<u16> }

impl ConfigBuilder {
    fn host(mut self, h: impl Into<String>) -> Self { self.host = Some(h.into()); self }
    fn port(mut self, p: u16) -> Self { self.port = Some(p); self }
    fn build(self) -> Config { Config { host: self.host.unwrap_or_default(), port: self.port.unwrap_or(80) } }
}
```
