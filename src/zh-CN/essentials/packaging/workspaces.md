### Cargo 工作区

Cargo 工作区是一组共享同一个 `Cargo.lock` 文件和输出目录的包。工作区非常适合管理拆分为多个相关 crate 的大型项目，可以在保持单一构建配置的同时实现代码重用。

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
├── Cargo.lock          # 共享的 lock 文件
├── target/             # 共享的构建目录
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

**1. 统一的依赖管理**：所有 crate 共享同一个 `Cargo.lock`

```toml
# 而不是每个 crate 都有不同的版本：
# crate1: serde = "1.0.150"
# crate2: serde = "1.0.152"

# 工作区确保所有 crate 使用相同的版本
```

**2. 共享的构建产物**：单个 `target/` 目录节省了磁盘空间和构建时间

```bash
# 没有工作区：多个 target 目录
crate1/target/
crate2/target/
crate3/target/

# 有工作区：单个共享目录
target/
```

**3. 原子版本更新**：一次性更新所有 crate

```bash
# 更新所有工作区依赖项
cargo update
```

### 构建工作区 Crate

```bash
# 构建所有工作区成员
cargo build

# 构建特定的 crate
cargo build -p my-core

# 使用发布优化构建所有成员
cargo build --release --workspace

# 运行所有成员的测试
cargo test --workspace

# 运行特定 crate 的二进制文件
cargo run -p my-cli
```

### 工作区依赖项

**共享依赖项**（定义一次，随处使用）：

```toml
[workspace]
members = ["crate1", "crate2"]

[workspace.dependencies]
serde = "1.0"
tokio = "1.0"

# 在 crate1/Cargo.toml 和 crate2/Cargo.toml 中：
[dependencies]
serde = { workspace = true }
tokio = { workspace = true }
```

**本地依赖项**（依赖于其他工作区成员）：

```toml
# 在 cli/Cargo.toml 中
[dependencies]
my-core = { path = "../core" }
my-utils = { path = "../utils", version = "0.1.0" }
```

### 工作区元数据

在所有包之间共享元数据：

```toml
[workspace]
members = ["crate1", "crate2"]

[workspace.package]
edition = "2021"
authors = ["Your Name <you@example.com>"]
license = "MIT"
repository = "https://github.com/user/repo"

# 在成员的 Cargo.toml 中：
[package]
name = "crate1"
version = "0.1.0"
edition.workspace = true
authors.workspace = true
license.workspace = true
```

### 真实世界示例

```toml
# 根 Cargo.toml
[workspace]
members = [
    "backend",
    "frontend-api",
    "shared-models",
    "database",
]
resolver = "2"

[workspace.dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
sqlx = { version = "0.7", features = ["postgres"] }

[workspace.package]
edition = "2021"
authors = ["Team <team@company.com>"]
license = "MIT"
```

```toml
# backend/Cargo.toml
[package]
name = "backend"
version = "0.1.0"
edition.workspace = true

[dependencies]
tokio = { workspace = true }
shared-models = { path = "../shared-models" }
database = { path = "../database" }
axum = "0.7"
```

```toml
# shared-models/Cargo.toml
[package]
name = "shared-models"
version = "0.1.0"
edition.workspace = true

[dependencies]
serde = { workspace = true }
```

### 开发工作流

```bash
# 克隆并构建整个工作区
git clone https://github.com/user/project
cd project
cargo build

# 在特定的 crate 上工作
cd backend
cargo check
cargo test

# 从工作区根目录运行
cd ..
cargo run -p backend

# 更新所有 crate 的依赖项
cargo update

# 检查所有 crate
cargo check --workspace
```

### 发布工作区 Crate

工作区中的 Crate 可以独立发布：

```bash
# 从 crate 目录发布
cd shared-models
cargo publish

cd ../backend
cargo publish

# 或者从工作区根目录发布
cargo publish -p shared-models
cargo publish -p backend
```

### 虚拟工作区

一个没有根包的工作区（仅聚合其他包）：

```toml
# Cargo.toml (没有 [package] 部分)
[workspace]
members = [
    "crate1",
    "crate2",
]
```

### 默认成员

指定默认构建哪些成员：

```toml
[workspace]
members = ["crate1", "crate2", "crate3"]
default-members = ["crate1", "crate2"]

# cargo build 只会构建 crate1 和 crate2
# 除非指定了 --workspace
```

### 排除 Crate

```toml
[workspace]
members = ["crates/*"]
exclude = [
    "crates/experimental",
    "crates/deprecated",
]
```

### 工作区中的 Cargo 命令

```bash
# 构建命令
cargo build --workspace              # 构建所有成员
cargo build -p crate1               # 构建特定的 crate
cargo build --exclude crate2        # 构建除 crate2 之外的所有 crate

# 测试命令
cargo test --workspace              # 测试所有成员
cargo test -p crate1                # 测试特定的 crate

# 检查命令
cargo check --workspace             # 检查所有成员
cargo clippy --workspace            # 对所有成员进行 lint

# 清理
cargo clean                         # 清理整个工作区

# 文档
cargo doc --workspace --no-deps     # 为所有成员生成文档
cargo doc --open -p crate1          # 打开特定 crate 的文档
```

### 最佳实践

- **对相关的 crate 使用工作区** - 共享代码、工具和主应用程序
- **通过 `[workspace.dependencies]` 共享通用依赖项** 以保持一致性
- **每个工作区一个 `Cargo.lock`** 确保可复现的构建
- **按功能拆分** - 为核心逻辑、CLI、服务器等分离 crate
- **对工作区成员使用路径依赖**
- **保持工作区根目录最小化** - 通常只有工作区配置
- **根据需要一起或独立地对工作区成员进行版本控制**
- **在 README 中记录工作区结构**
- **对大型工作区使用 `default-members`** 以加快默认构建速度
- **定期使用 `cargo test --workspace` 测试整个工作区**
