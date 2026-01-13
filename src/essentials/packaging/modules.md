### How to call a function from a standard library:

```rust, editable
{{#include modules/src/stdlib/main.rs}}
```

### How to call a function from a library on Crates.io:

For external libraries, you can specify the external module in the Cargo.toml file.

```rust,noplaypen
{{#include modules/src/external-lib/main.rs}}
```

Cargo.toml file content

```toml
{{#include ./modules/src/external-lib/Cargo.toml}}
```

### Legacy solution for Rust 2015 edition or earlier 

On the Rust file you may call `extern crate PACKAGE_NAME` to use an external library. This is a handy solution when you want to use [Rust Playground](https://play.rust-lang.org/) like in this example. Otherwise, you would get an error when you execute the file.

```rust, editable
{{#include modules/src/rand/main.rs}}
```