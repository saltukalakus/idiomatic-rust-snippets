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

**2. 共享构建产物**：单一的 `target/` 目录节省了磁盘空间和构建时间

```bash
# 没有工作区：多个 target 目录
crate1/target/
crate2/target/
crate3/target/

# 有工作区：单一的共享目录
target/
```

**3. 原子化版本更新**：一次性更新所有 crate

```bash
# 更新所有工作区依赖
cargo update
```

### 构建工作区 Crates

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

### 工作区依赖

**共享依赖**（定义一次，随处使用）：

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

**本地依赖**（依赖于其他工作区成员）：

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

# 更新所有 crate 的依赖
cargo update

# 检查所有 crates
cargo check --workspace
```

### 发布工作区 Crates

工作区中的 Crates 可以独立发布：

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

一个没有根包的工作区（只聚合其他包）：

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

### 排除 Crates

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

- **为相关的 crates 使用工作区** - 共享代码、工具和主应用程序
- **通过 `[workspace.dependencies]` 共享通用依赖** 以保持一致性
- **每个工作区一个 `Cargo.lock`** 确保可复现的构建
- **按功能拆分** - 为核心逻辑、CLI、服务器等分离 crates
- **为工作区成员使用路径依赖**
- **保持工作区根目录最小化** - 通常只有工作区配置
- **根据需要一起或独立地对工作区成员进行版本控制**
- **在 README 中记录工作区结构**
- **为大型工作区使用 `default-members`** 以加快默认构建速度
- **定期使用 `cargo test --workspace` 测试整个工作区**

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
