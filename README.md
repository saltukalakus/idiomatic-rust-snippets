# [Idiomatic Rust Snippets](https://idiomatic-rust-snippets.org/)

Rust is known to have a steep learning curve. This book does not intend to explain every aspect of Rust; however, you may find it useful while learning Rust. This is a personal project and is not associated with the [Rust Foundation](https://foundation.rust-lang.org/). 

1. [Essentials](https://idiomatic-rust-snippets.org/essentials/intro.html) covers the basic aspects of the Rust programming language. They are explained with easy-to-understand examples.

2. [Design Patterns](https://idiomatic-rust-snippets.org/patterns/intro.html) section demonstrates several design patterns with minimalistic but complete projects.

3. [Algorithms](https://idiomatic-rust-snippets.org/algorithms/intro.html) section explains various popular algorithms with code samples in Rust. (WIP)

The project follows a minimalistic approach, where almost all pages are short with simple, easy-to-follow code samples. Most samples can run. External documentation is referenced when necessary for extended information.

**Note**: Pages with a **WIP** note haven't been reviewed yet, and the information in those pages may contain incorrect information.

### How to run the project locally.

This project uses Rust 1.87.0. The `rust-toolchain.toml` file ensures the correct version is used automatically when you run cargo commands.

**Using Make (recommended):**
```
git clone https://github.com/saltukalakus/idiomatic-rust-snippets
cd idiomatic-rust-snippets
make all        # Check Rust version, install dependencies and build
make serve      # Serve the book locally
```
***Available Make commands:***
- `make check-rust` - Verify Rust toolchain version
- `make install` - Install mdbook and mdbook-metadata
- `make build` - Build the book
- `make serve` - Build and serve the book locally
- `make clean` - Clean build artifacts
- `make all` - Check Rust, install dependencies and build

**Manual installation:**
```
git clone https://github.com/saltukalakus/idiomatic-rust-snippets
cd idiomatic-rust-snippets
rustup install 1.87.0  # Install the correct Rust version (optional, rust-toolchain.toml handles this)
cargo install mdbook --version 0.4.52
cargo install mdbook-metadata --version 0.1.1
mdbook build
mdbook serve
```

Last but not least, please like ⭐️⭐️⭐️ [the project](https://github.com/saltukalakus/idiomatic-rust-snippets) on Github if you find it useful. This is a great motivation to keep the project active.
