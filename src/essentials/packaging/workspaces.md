### Cargo Workspaces

A Cargo workspace is a set of packages that share the same `Cargo.lock` file and output directory. Workspaces are ideal for managing large projects split into multiple related crates, enabling code reuse while maintaining a single build configuration.

### Creating a Workspace

Create a workspace by defining a `Cargo.toml` file at the workspace root:

```toml
[workspace]
members = [
    "crate1",
    "crate2",
    "crate3",
]

# Optional: explicitly exclude directories
exclude = [
    "target",
    "old-crates",
]
```

### Directory Structure

```
my-workspace/
├── Cargo.toml          # Workspace manifest
├── Cargo.lock          # Shared lock file
├── target/             # Shared build directory
├── crate1/
│   ├── Cargo.toml      # Package manifest
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

### Example Setup

**Workspace `Cargo.toml`:**

```toml
[workspace]
members = [
    "core",
    "cli",
    "server",
]

# Shared dependencies across workspace
[workspace.dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
```

**Package `core/Cargo.toml`:**

```toml
[package]
name = "my-core"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { workspace = true }
```

**Package `cli/Cargo.toml`:**

```toml
[package]
name = "my-cli"
version = "0.1.0"
edition = "2021"

[dependencies]
my-core = { path = "../core" }
serde = { workspace = true }
```

### Benefits of Workspaces

**1. Unified Dependency Management**: All crates share the same `Cargo.lock`

```toml
# Instead of each crate having different versions:
# crate1: serde = "1.0.150"
# crate2: serde = "1.0.152"

# Workspace ensures all use the same version
```

**2. Shared Build Artifacts**: Single `target/` directory saves disk space and build time

```bash
# Without workspace: multiple target directories
crate1/target/
crate2/target/
crate3/target/

# With workspace: single shared directory
target/
```

**3. Atomic Version Updates**: Update all crates at once

```bash
# Update all workspace dependencies
cargo update
```

### Building Workspace Crates

```bash
# Build all workspace members
cargo build

# Build specific crate
cargo build -p my-core

# Build all members with release optimization
cargo build --release --workspace

# Run tests for all members
cargo test --workspace

# Run specific crate's binary
cargo run -p my-cli
```

### Workspace Dependencies

**Shared dependencies** (defined once, used everywhere):

```toml
[workspace]
members = ["crate1", "crate2"]

[workspace.dependencies]
serde = "1.0"
tokio = "1.0"

# In crate1/Cargo.toml and crate2/Cargo.toml:
[dependencies]
serde = { workspace = true }
tokio = { workspace = true }
```

**Local dependencies** (depend on other workspace members):

```toml
# In cli/Cargo.toml
[dependencies]
my-core = { path = "../core" }
my-utils = { path = "../utils", version = "0.1.0" }
```

### Workspace Metadata

Share metadata across all packages:

```toml
[workspace]
members = ["crate1", "crate2"]

[workspace.package]
edition = "2021"
authors = ["Your Name <you@example.com>"]
license = "MIT"
repository = "https://github.com/user/repo"

# In member Cargo.toml:
[package]
name = "crate1"
version = "0.1.0"
edition.workspace = true
authors.workspace = true
license.workspace = true
```

### Real-World Example

```toml
# Root Cargo.toml
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

### Development Workflow

```bash
# Clone and build entire workspace
git clone https://github.com/user/project
cd project
cargo build

# Work on specific crate
cd backend
cargo check
cargo test

# Run from workspace root
cd ..
cargo run -p backend

# Update dependencies for all crates
cargo update

# Check all crates
cargo check --workspace
```

### Publishing Workspace Crates

Crates in a workspace can be published independently:

```bash
# Publish from crate directory
cd shared-models
cargo publish

cd ../backend
cargo publish

# Or publish from workspace root
cargo publish -p shared-models
cargo publish -p backend
```

### Virtual Workspaces

A workspace without a root package (only aggregates other packages):

```toml
# Cargo.toml (no [package] section)
[workspace]
members = [
    "crate1",
    "crate2",
]
```

### Default Members

Specify which members to build by default:

```toml
[workspace]
members = ["crate1", "crate2", "crate3"]
default-members = ["crate1", "crate2"]

# cargo build will only build crate1 and crate2
# unless --workspace is specified
```

### Excluding Crates

```toml
[workspace]
members = ["crates/*"]
exclude = [
    "crates/experimental",
    "crates/deprecated",
]
```

### Cargo Commands in Workspaces

```bash
# Build commands
cargo build --workspace              # Build all members
cargo build -p crate1               # Build specific crate
cargo build --exclude crate2        # Build all except crate2

# Test commands
cargo test --workspace              # Test all members
cargo test -p crate1                # Test specific crate

# Check commands
cargo check --workspace             # Check all members
cargo clippy --workspace            # Lint all members

# Clean
cargo clean                         # Clean entire workspace

# Documentation
cargo doc --workspace --no-deps     # Generate docs for all members
cargo doc --open -p crate1          # Open docs for specific crate
```

### Best Practices

- **Use workspaces for related crates** - shared code, tools, and main application
- **Share common dependencies** via `[workspace.dependencies]` for consistency
- **One `Cargo.lock`** per workspace ensures reproducible builds
- **Split by functionality** - separate crates for core logic, CLI, server, etc.
- **Use path dependencies** for workspace members
- **Keep workspace root minimal** - usually just workspace configuration
- **Version workspace members together** or independently based on needs
- **Document the workspace structure** in README
- **Use `default-members`** for large workspaces to speed up default builds
- **Test the entire workspace** regularly with `cargo test --workspace`
