### What is features?

In Rust, **features** are a way to conditionally compile code based on specified configurations. They allow you to enable or disable certain functionalities in your crate or its dependencies. Features are defined in the Cargo.toml file and can be used to control optional dependencies, conditional compilation.

### Defining Features
You define features in your crate's Cargo.toml file under the [features] section. Here’s an example:

```toml
[features]
default = ["feature1", "feature2"]
feature1 = []
feature2 = ["dependency1/featureA"]
```

**default**: This is a special feature that is enabled by default when your crate is used as a dependency. It can include other features.<br/>
**feature1 and feature2**: These are custom features that can be enabled or disabled by the user.<br/>

### Using Features in Code

You can use features in your Rust code with conditional compilation attributes like `#[cfg(feature = "feature_name")]` and `#[cfg_attr(feature = "feature_name", some_attribute)]`.

```rust, noplaypen
#[cfg(feature = "feature1")]
fn feature1_function() {
    // Code that is only compiled if "feature1" is enabled
}

#[cfg(feature = "feature2")]
fn feature2_function() {
    // Code that is only compiled if "feature2" is enabled
}
```

When you depend on a crate, you can enable features in your Cargo.toml file:

```toml
[dependencies]
my_crate = { version = "1.0", features = ["feature1", "feature2"] }
```

If you want to depend on a crate but do not want to enable its default features, you can use the `default-features = false` option:

```toml
[dependencies]
my_crate = { version = "1.0", default-features = false, features = ["feature1"] }
```

`default-features = false` disables the **default** features.
`features = ["feature1"]` enables the **feature1** specifically.

`features`: Allow conditional compilation and optional dependencies.<br/>
`default` attribute: Default features is enabled by default unless explicitly disabled.<br/>

Custom Features can be added in Cargo.toml and enabled or disabled as needed. You can use `#[cfg(feature = "feature_name")]` at the top of the function to include/exclude from teh code based on the feature availability.<br/>