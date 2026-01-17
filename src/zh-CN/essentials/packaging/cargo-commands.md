````markdown
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

````
