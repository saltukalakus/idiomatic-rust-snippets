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
```

**`cargo fmt`** - 格式化代码：

```bash
# 检查格式问题
cargo fmt -- --check

# 格式化所有工作区成员
cargo fmt --all
```

### 依赖管理

**`cargo add`** - 添加依赖项：

```bash
# 添加最新版本
cargo add serde

# 添加特定版本
cargo add serde --vers 1.0

# 添加开发依赖
cargo add serde --dev

# 添加构建依赖
cargo add serde --build

# 从 git 添加
cargo add serde --git https://github.com/serde-rs/serde
```

**`cargo rm`** - 移除依赖项：

```bash
# 移除依赖项
cargo rm serde

# 移除开发依赖
cargo rm serde --dev
```

**`cargo update`** - 更新依赖项：

```bash
# 更新所有依赖项
cargo update

# 更新特定包
cargo update -p serde
```

**`cargo tree`** - 显示依赖关系树：

```bash
# 显示依赖关系树
cargo tree

# 显示特定包的依赖关系
cargo tree -p serde

# 反向显示依赖关系
cargo tree -i serde
```

### 发布与文档

**`cargo doc`** - 构建文档：

```bash
# 构建文档
cargo doc

# 在浏览器中打开文档
cargo doc --open

# 为所有工作区成员构建文档
cargo doc --workspace
```

**`cargo publish`** - 发布到 crates.io：

```bash
# 执行试运行
cargo publish --dry-run

# 发布包
cargo publish

# 发布特定版本
cargo publish --token <token>
```

**`cargo install`** - 从 crates.io 安装二进制文件：

```bash
# 安装二进制文件
cargo install ripgrep

# 强制重新安装
cargo install ripgrep --force

# 从 git 安装
cargo install --git https://github.com/BurntSushi/ripgrep.git
```

**`cargo owner`** - 管理 crate 所有者：

```bash
# 添加所有者
cargo owner --add github_username

# 移除所有者
cargo owner --remove github_username

# 列出所有者
cargo owner --list
```

### 其他命令

**`cargo clean`** - 清理构建目录：

```bash
# 清理构建目录
cargo clean

# 清理发布构建
cargo clean --release

# 清理文档
cargo clean --doc
```

**`cargo search`** - 搜索 crates.io：

```bash
# 搜索包
cargo search serde

# 限制结果数量
cargo search serde --limit 5
```

**`cargo login`** - 登录到 crates.io：

```bash
# 使用 API 令牌登录
cargo login <token>
```

**`cargo logout`** - 从 crates.io 注销：

```bash
# 注销
cargo logout
```

**`cargo metadata`** - 以 JSON 格式输出元数据：

```bash
# 获取元数据
cargo metadata

# 无依赖
cargo metadata --no-deps

# 指定格式版本
cargo metadata --format-version 1
```

**`cargo locate-project`** - 查找 `Cargo.toml`：

```bash
# 查找 `Cargo.toml`
cargo locate-project

# 查找工作区根目录
cargo locate-project --workspace
```

**`cargo pkgid`** - 显示包 ID：

```bash
# 显示当前包的 ID
cargo pkgid

# 显示特定包的 ID
cargo pkgid serde
```

**`cargo verify-project`** - 检查项目清单：

```bash
# 验证项目
cargo verify-project
```

**`cargo vendor`** - 将依赖项打包到 `vendor` 目录：

```bash
# 将依赖项打包
cargo vendor

# 同步 `vendor` 目录
cargo vendor --sync
```

**`cargo help`** - 显示帮助信息：

```bash
# 显示所有命令
cargo help

# 显示特定命令的帮助
cargo help build
````
