### Cargo Commands

Cargo is Rust's build system and package manager. It provides a comprehensive set of commands for building, testing, documenting, and publishing Rust projects. Understanding these commands is essential for effective Rust development.

### Essential Commands

**`cargo new`** - Create a new project:

```bash
# Create a binary project
cargo new my-project

# Create a library project
cargo new my-lib --lib

# With specific edition
cargo new my-project --edition 2021
```

**`cargo build`** - Compile the project:

```bash
# Debug build (fast compilation, slower runtime)
cargo build

# Release build (optimized, slow compilation, fast runtime)
cargo build --release

# Build specific binary
cargo build --bin my-binary

# Build all workspace members
cargo build --workspace
```

**`cargo run`** - Build and run:

```bash
# Run default binary
cargo run

# Run with arguments
cargo run -- --arg1 value1

# Run specific binary
cargo run --bin my-binary

# Run with release optimization
cargo run --release
```

**`cargo test`** - Run tests:

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests matching pattern
cargo test pattern

# Show output from passing tests
cargo test -- --nocapture

# Run tests with specific number of threads
cargo test -- --test-threads=1

# Run only doc tests
cargo test --doc
```

**`cargo check`** - Fast compilation without code generation:

```bash
# Check for errors without building
cargo check

# Much faster than cargo build
# Useful during development

# Check all workspace members
cargo check --workspace
```

### Code Quality Commands

**`cargo clippy`** - Linting tool:

```bash
# Run clippy lints
cargo clippy

# Treat warnings as errors
cargo clippy -- -D warnings

# Fix automatically fixable issues
cargo clippy --fix

# Run on all workspace members
cargo clippy --workspace
```

**`cargo fmt`** - Format code:

```bash
# Format all Rust files
cargo fmt

# Check formatting without modifying files
cargo fmt --check

# Format specific file
cargo fmt path/to/file.rs
```

**`cargo fix`** - Automatically fix compiler warnings:

```bash
# Fix automatically fixable issues
cargo fix

# Allow making changes to dirty working directory
cargo fix --allow-dirty

# Fix for a different edition
cargo fix --edition
```

### Dependency Management

**`cargo add`** - Add dependencies (requires cargo-edit):

```bash
# Add latest version
cargo add serde

# Add with features
cargo add tokio --features full

# Add dev dependency
cargo add --dev mockito

# Add build dependency
cargo add --build cc
```

**`cargo remove`** - Remove dependencies:

```bash
# Remove dependency
cargo remove serde
```

**`cargo update`** - Update dependencies:

```bash
# Update all dependencies
cargo update

# Update specific dependency
cargo update -p serde

# Update to latest compatible versions
cargo update --aggressive
```

**`cargo tree`** - Display dependency tree:

```bash
# Show dependency tree
cargo tree

# Show dependencies for specific package
cargo tree -p my-crate

# Show inverse dependencies (what depends on a crate)
cargo tree -i tokio

# Limit depth
cargo tree --depth 2
```

### Documentation Commands

**`cargo doc`** - Generate documentation:

```bash
# Generate docs for project
cargo doc

# Generate and open in browser
cargo doc --open

# Include dependencies
cargo doc --no-deps

# Generate for all workspace members
cargo doc --workspace
```

**`cargo rustdoc`** - Pass flags to rustdoc:

```bash
# Generate docs with extra flags
cargo rustdoc -- --document-private-items
```

### Publishing Commands

**`cargo publish`** - Publish to crates.io:

```bash
# Dry run (test packaging without publishing)
cargo publish --dry-run

# Actually publish
cargo publish

# Publish specific crate in workspace
cargo publish -p my-crate
```

**`cargo package`** - Create distributable package:

```bash
# Package crate
cargo package

# List packaged files without creating archive
cargo package --list
```

**`cargo yank`** - Mark version as unavailable:

```bash
# Yank a version
cargo yank --vers 1.0.0

# Un-yank
cargo yank --vers 1.0.0 --undo
```

### Utility Commands

**`cargo clean`** - Remove build artifacts:

```bash
# Clean target directory
cargo clean

# Clean specific package
cargo clean -p my-crate

# Clean release artifacts only
cargo clean --release
```

**`cargo search`** - Search crates.io:

```bash
# Search for crates
cargo search serde

# Limit results
cargo search serde --limit 10
```

**`cargo install`** - Install binary crates:

```bash
# Install from crates.io
cargo install ripgrep

# Install from git
cargo install --git https://github.com/user/repo

# Install with specific features
cargo install cargo-edit --features vendored-openssl

# Install local crate
cargo install --path .
```

**`cargo uninstall`** - Remove installed binaries:

```bash
# Uninstall
cargo uninstall ripgrep
```

### Benchmarking and Profiling

**`cargo bench`** - Run benchmarks:

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench benchmark_name

# Save baseline
cargo bench -- --save-baseline my-baseline
```

**`cargo expand`** - Expand macros (requires cargo-expand):

```bash
# Expand macros in current crate
cargo expand

# Expand specific module
cargo expand module::path
```

### Advanced Commands

**`cargo metadata`** - Machine-readable project info:

```bash
# Output project metadata as JSON
cargo metadata

# Format nicely
cargo metadata --format-version 1 | jq
```

**`cargo vendor`** - Vendor dependencies locally:

```bash
# Download all dependencies to vendor/
cargo vendor

# Use vendored dependencies
mkdir .cargo
echo '[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor"' > .cargo/config.toml
```

**`cargo fetch`** - Download dependencies without building:

```bash
# Fetch dependencies
cargo fetch

# Useful for CI caching
```

### Cargo Watch (External Tool)

**`cargo watch`** - Auto-rebuild on file changes:

```bash
# Install cargo-watch
cargo install cargo-watch

# Watch and run checks
cargo watch -x check

# Watch and run tests
cargo watch -x test

# Chain commands
cargo watch -x check -x test -x run
```

### Configuration

**View configuration**:

```bash
# Show all config values
cargo config get

# Show specific config
cargo config get build.target
```

**Set configuration**:

```bash
# Set config value
cargo config set build.jobs 4
```

### Cargo with Specific Toolchains

```bash
# Use specific Rust version
cargo +stable build
cargo +nightly test
cargo +1.70.0 build

# Use toolchain from rust-toolchain.toml
cargo build  # Uses version specified in file
```

### Useful Command Combinations

```bash
# Quick development cycle
cargo check && cargo test && cargo clippy

# Pre-commit checks
cargo fmt --check && cargo clippy -- -D warnings && cargo test

# Full CI pipeline
cargo fmt --check && \
cargo clippy --all-targets --all-features -- -D warnings && \
cargo test --all-features && \
cargo doc --no-deps

# Profile release build
cargo build --release && time ./target/release/my-app

# Clean and rebuild
cargo clean && cargo build --release
```

### Environment Variables

```bash
# Increase parallel jobs
CARGO_BUILD_JOBS=8 cargo build

# Verbose output
CARGO_LOG=debug cargo build

# Use offline mode
CARGO_NET_OFFLINE=true cargo build

# Custom target directory
CARGO_TARGET_DIR=/tmp/target cargo build
```

### Best Practices

- **Use `cargo check`** during development - much faster than build
- **Run `cargo clippy`** regularly - catches common mistakes
- **Format code with `cargo fmt`** - maintains consistency
- **Use `cargo test`** frequently - catch regressions early
- **Generate docs with `cargo doc`** - keeps documentation up to date
- **Use `cargo tree`** to understand dependency graph
- **Clean occasionally** with `cargo clean` if builds act strange
- **Install `cargo-watch`** for automatic rebuilds during development
- **Use `--release` for benchmarking** - debug builds are much slower
- **Check `cargo --help`** and `cargo <command> --help` for more options
