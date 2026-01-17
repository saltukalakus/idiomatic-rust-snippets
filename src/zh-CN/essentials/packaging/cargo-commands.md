### Cargo 命令

Cargo 是 Rust 的构建系统和包管理器。它提供了一套全面的命令，用于构建、测试、记录和发布 Rust 项目。了解这些命令对于高效的 Rust 开发至关重要。

### 基本命令

**`cargo new`** - 创建一个新项目：

```bash
# 创建一个二进制项目
cargo new my-project

# 创建一个库项目
cargo new my-lib --lib

# 指定版本
cargo new my-project --edition 2021
```

**`cargo build`** - 编译项目：

```bash
# 调试构建（编译速度快，运行速度慢）
cargo build

# 发布构建（优化，编译速度慢，运行速度快）
cargo build --release

# 构建特定的二进制文件
cargo build --bin my-binary

# 构建所有工作区成员
cargo build --workspace
```

**`cargo run`** - 构建并运行：

```bash
# 运行默认的二进制文件
cargo run

# 带参数运行
cargo run -- --arg1 value1

# 运行特定的二进制文件
cargo run --bin my-binary

# 使用发布优化运行
cargo run --release
```

**`cargo test`** - 运行测试：

```bash
# 运行所有测试
cargo test

# 运行特定的测试
cargo test test_name

# 运行匹配模式的测试
cargo test pattern

# 显示通过测试的输出
cargo test -- --nocapture

# 使用特定数量的线程运行测试
cargo test -- --test-threads=1

# 只运行文档测试
cargo test --doc
```

**`cargo check`** - 快速编译而不生成代码：

```bash
# 检查错误而不构建
cargo check

# 比 cargo build 快得多
# 在开发过程中很有用

# 检查所有工作区成员
cargo check --workspace
```

### 代码质量命令

**`cargo clippy`** - Linting 工具：

```bash
# 运行 clippy lints
cargo clippy

# 将警告视为错误
cargo clippy -- -D warnings

# 自动修复可修复的问题
cargo clippy --fix

# 对所有工作区成员运行 clippy
cargo clippy --workspace
```

**`cargo fmt`** - 格式化代码：

```bash
# 格式化所有 Rust 文件
cargo fmt

# 检查格式而不修改文件
cargo fmt --check

# 格式化特定文件
cargo fmt path/to/file.rs
```

**`cargo fix`** - 自动修复编译器警告：

```bash
# 自动修复可修复的问题
cargo fix

# 允许在有未提交更改的工作目录中进行更改
cargo fix --allow-dirty

# 针对不同版本进行修复
cargo fix --edition
```

### 依赖管理

**`cargo add`** - 添加依赖（需要 cargo-edit）：

```bash
# 添加最新版本
cargo add serde

# 添加带特性的依赖
cargo add tokio --features full

# 添加开发依赖
cargo add --dev mockito

# 添加构建依赖
cargo add --build cc
```

**`cargo remove`** - 移除依赖：

```bash
# 移除依赖
cargo remove serde
```

**`cargo update`** - 更新依赖：

```bash
# 更新所有依赖
cargo update

# 更新特定依赖
cargo update -p serde

# 更新到最新的兼容版本
cargo update --aggressive
```

**`cargo tree`** - 显示依赖树：

```bash
# 显示依赖树
cargo tree

# 显示特定包的依赖
cargo tree -p my-crate

# 显示反向依赖（哪些包依赖于此 crate）
cargo tree -i tokio

# 限制深度
cargo tree --depth 2
```

### 文档命令

**`cargo doc`** - 生成文档：

```bash
# 为项目生成文档
cargo doc

# 生成并在浏览器中打开
cargo doc --open

# 不包含依赖
cargo doc --no-deps

# 为所有工作区成员生成文档
cargo doc --workspace
```

**`cargo rustdoc`** - 传递标志给 rustdoc：

```bash
# 使用额外标志生成文档
cargo rustdoc -- --document-private-items
```

### 发布命令

**`cargo publish`** - 发布到 crates.io：

```bash
# 空运行（测试打包而不发布）
cargo publish --dry-run

# 实际发布
cargo publish

# 发布工作区中的特定 crate
cargo publish -p my-crate
```

**`cargo package`** - 创建可分发的包：

```bash
# 打包 crate
cargo package

# 列出打包的文件而不创建归档
cargo package --list
```

**`cargo yank`** - 标记版本为不可用：

```bash
# 废弃一个版本
cargo yank --vers 1.0.0

# 取消废弃
cargo yank --vers 1.0.0 --undo
```

### 工具命令

**`cargo clean`** - 移除构建产物：

```bash
# 清理 target 目录
cargo clean

# 清理特定包
cargo clean -p my-crate

# 只清理发布产物
cargo clean --release
```

**`cargo search`** - 搜索 crates.io：

```bash
# 搜索 crates
cargo search serde

# 限制结果数量
cargo search serde --limit 10
```

**`cargo install`** - 安装二进制 crates：

```bash
# 从 crates.io 安装
cargo install ripgrep

# 从 git 安装
cargo install --git https://github.com/user/repo

# 使用特定特性安装
cargo install cargo-edit --features vendored-openssl

# 安装本地 crate
cargo install --path .
```

**`cargo uninstall`** - 移除已安装的二进制文件：

```bash
# 卸载
cargo uninstall ripgrep
```

### 基准测试和性能分析

**`cargo bench`** - 运行基准测试：

```bash
# 运行所有基准测试
cargo bench

# 运行特定的基准测试
cargo bench benchmark_name

# 保存基线
cargo bench -- --save-baseline my-baseline
```

**`cargo expand`** - 展开宏（需要 cargo-expand）：

```bash
# 展开当前 crate 中的宏
cargo expand

# 展开特定模块
cargo expand module::path
```

### 高级命令

**`cargo metadata`** - 机器可读的项目信息：

```bash
# 以 JSON 格式输出项目元数据
cargo metadata

# 格式化输出
cargo metadata --format-version 1 | jq
```

**`cargo vendor`** - 将依赖项本地化：

```bash
# 将所有依赖项下载到 vendor/
cargo vendor

# 使用本地化的依赖项
mkdir .cargo
echo '[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor"' > .cargo/config.toml
```

**`cargo fetch`** - 下载依赖项而不构建：

```bash
# 获取依赖项
cargo fetch

# 对 CI 缓存很有用
```

### Cargo Watch（外部工具）

**`cargo watch`** - 文件更改时自动重新构建：

```bash
# 安装 cargo-watch
cargo install cargo-watch

# 监视并运行检查
cargo watch -x check

# 监视并运行测试
cargo watch -x test

# 链接命令
cargo watch -x check -x test -x run
```

### 配置

**查看配置**：

```bash
# 显示所有配置值
cargo config get

# 显示特定配置
cargo config get build.target
```

**设置配置**：

```bash
# 设置配置值
cargo config set build.jobs 4
```

### 使用特定工具链的 Cargo

```bash
# 使用特定的 Rust 版本
cargo +stable build
cargo +nightly test
cargo +1.70.0 build

# 使用 rust-toolchain.toml 中的工具链
cargo build  # 使用文件中指定的版本
```

### 有用的命令组合

```bash
# 快速开发周期
cargo check && cargo test && cargo clippy

# 提交前检查
cargo fmt --check && cargo clippy -- -D warnings && cargo test

# 完整的 CI 流水线
cargo fmt --check && \
cargo clippy --all-targets --all-features -- -D warnings && \
cargo test --all-features && \
cargo doc --no-deps

# 分析发布构建
cargo build --release && time ./target/release/my-app

# 清理并重新构建
cargo clean && cargo build --release
```

### 环境变量

```bash
# 增加并行作业数
CARGO_BUILD_JOBS=8 cargo build

# 详细输出
CARGO_LOG=debug cargo build

# 使用离线模式
CARGO_NET_OFFLINE=true cargo build

# 自定义 target 目录
CARGO_TARGET_DIR=/tmp/target cargo build
```

### 最佳实践

- **在开发期间使用 `cargo check`** - 比构建快得多
- **定期运行 `cargo clippy`** - 捕获常见错误
- **使用 `cargo fmt` 格式化代码** - 保持一致性
- **频繁使用 `cargo test`** - 及早发现回归
- **使用 `cargo doc` 生成文档** - 保持文档最新
- **使用 `cargo tree` 理解依赖图**
- **如果构建出现异常，偶尔使用 `cargo clean` 清理**
- **安装 `cargo-watch`** 以在开发期间自动重新构建
- **使用 `--release` 进行基准测试** - 调试构建要慢得多
- **查看 `cargo --help` 和 `cargo <command> --help`** 获取更多选项
