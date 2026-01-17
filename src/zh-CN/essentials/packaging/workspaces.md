````markdown
### Cargo 工作区

Cargo 工作区是一组共享相同 `Cargo.lock` 文件和输出目录的包。工作区非常适合管理拆分为多个相关 crate 的大型项目，可在保持单一构建配置的同时实现代码重用。

### 创建工作区

通过在工作区根目录中定义一个 `Cargo.toml` 文件来创建工作区：

```toml
[workspace]
members = [
    "crate1",
    "crate2",
    "crate3",
]

# 可选：明确排除目录
exclude = [
    "target",
    "old-crates",
]
```

### 目录结构

```
my-workspace/
├── Cargo.toml          # 工作区清单
├── Cargo.lock          # 共享锁文件
├── target/             # 共享构建目录
├── crate1/
│   ├── Cargo.toml      # 包清单
│   └── src/
│       └── lib.rs
├── crate2/
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
└── crate3/
    ├── Cargo.toml
    └── src/
        └── main.rs
```

### 示例设置

**工作区 `Cargo.toml`：**

```toml
[workspace]
members = [
    "core",
    "cli",
    "server",
]

# 跨工作区共享的依赖项
[workspace.dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
```

**包 `core/Cargo.toml`：**

```toml
[package]
name = "my-core"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { workspace = true }
```

**包 `cli/Cargo.toml`：**

```toml
[package]
name = "my-cli"
version = "0.1.0"
edition = "2021"

[dependencies]
my-core = { path = "../core" }
serde = { workspace = true }
```

### 工作区的优势

**1. 统一的依赖管理**：所有 crate 共享相同的 `Cargo.lock`，确保所有 crate 都使用相同的版本。

**2. 共享构建产物**：单一的 `target/` 目录节省了磁盘空间和构建时间。

**3. 原子化发布**：`cargo publish` 将按拓扑顺序发布工作区中的所有 crate。

**4. 跨 Crate 重构**：IDE 和工具可以同时对所有 crate 进行操作。

### 常用命令

在工作区中运行 Cargo 命令时，它们会应用于整个工作区。

```bash
# 构建工作区中的所有 crate
cargo build --workspace

# 运行特定的二进制 crate
cargo run -p my-cli-crate

# 测试所有 crate
cargo test --workspace

# 测试特定的 crate
cargo test -p my-lib-crate

# 将 clippy 应用于所有 crate
cargo clippy --workspace
```

### 虚拟工作区

虚拟工作区是一个 `Cargo.toml` 文件，它只定义了一个 `[workspace]` 部分，而没有 `[package]` 部分。这对于组织一组不属于同一个包的 crate 非常有用。

**虚拟 `Cargo.toml` 示例：**

```toml
[workspace]
members = [
    "project1",
    "project2",
]
resolver = "2" # 推荐用于工作区
```

### 依赖项覆盖

你可以在工作区级别覆盖依赖项，这对于测试本地或分叉版本的 crate 非常有用。

```toml
[patch.crates-io]
# 临时使用本地版本的 `serde`
serde = { path = "../forks/serde" }
```

### 最佳实践

- **使用 `workspace.dependencies`**：在 `[workspace.dependencies]` 中定义共享依赖项，以确保版本一致。
- **使用 `workspace.package`**：在 `[workspace.package]` 中定义共享的元数据，如 `authors`、`license` 和 `repository`。
- **使用路径依赖**：对于工作区内的 crate，使用 `path = "../other-crate"` 引用它们。
- **运行 `cargo check`**：在开发过程中使用 `cargo check --workspace` 进行快速编译检查。
- **CI/CD 设置**：在持续集成管道中，使用 `--workspace` 标志来构建和测试所有 crate。
````
