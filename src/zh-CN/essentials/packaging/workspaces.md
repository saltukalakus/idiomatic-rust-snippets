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

**1. 统一的依赖管理**：所有 crate 共享相同的 `Cargo.lock`

```toml
# 而不是每个 crate 都有不同的版本：
# crate1: serde = "1.0.150"
# crate2: serde = "1.0.152"

# 工作区确保所有 crate 都使用相同的版本
```

**2. 共享的构建产物**：单个 `target/` 目录可节省磁盘空间和构建时间
````
