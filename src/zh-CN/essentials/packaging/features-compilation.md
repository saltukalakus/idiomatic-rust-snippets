### 什么是特性？

特性（Features）是一种根据指定配置有条件地编译代码的方法。它们允许您在您的 crate 或其依赖项中启用或禁用某些功能。特性在 Cargo.toml 文件中定义，可用于控制可选依赖项和条件编译。

### 定义特性
您可以在您的 crate 的 Cargo.toml 文件的 [features] 部分定义特性。

```toml
[features]
default = ["feature1", "feature2"]
feature1 = []
feature2 = ["dependency1/featureA"]
```

**default**: 这是一个特殊的特性，当您的 crate 作为依赖项使用时，默认启用。它可以包含其他特性。<br/>
**feature1 和 feature2**: 这些是可由用户启用或禁用的自定义特性。<br/>

### 在代码中使用特性

您可以在 Rust 代码中使用条件编译属性，如 `#[cfg(feature = "feature_name")]` 和 `#[cfg_attr(feature = "feature_name", some_attribute)]`。

```rust, noplaypen
#[cfg(feature = "feature1")]
fn feature1_function() {
    // 仅在 "feature1" 启用时编译的代码
}

#[cfg(feature = "feature2")]
fn feature2_function() {
    // 仅在 "feature2" 启用时编译的代码
}
```

当您依赖一个 crate 时，可以在您的 Cargo.toml 文件中启用特性：

```toml
[dependencies]
my_crate = { version = "1.0", features = ["feature1", "feature2"] }
```

如果您想依赖一个 crate 但不想启用其默认特性，可以使用 `default-features = false` 选项：

```toml
[dependencies]
my_crate = { version = "1.0", default-features = false, features = ["feature1"] }
```

`default-features = false` 禁用 **default** 特性。
`features = ["feature1"]` 专门启用 **feature1**。

`features`: 允许条件编译和可选依赖项。<br/>
`default` 属性: 默认特性默认启用，除非明确禁用。<br/>

自定义特性可以在 Cargo.toml 中添加，并根据需要启用或禁用。您可以在函数顶部使用 `#[cfg(feature = "feature_name")]` 来根据特性可用性在代码中包含/排除函数。<br/>
