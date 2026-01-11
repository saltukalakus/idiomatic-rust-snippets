# [Idiomatic Rust Snippets](https://idiomatic-rust-snippets.org/)

Rust is known to have a steep learning curve. This book does not intend to explain every aspect; however, you may find it useful while learning Rust. This is a personal project and is not associated with the [Rust Foundation](https://foundation.rust-lang.org/). 

1. [Essentials](./essentials/intro.md) covers the basic aspects of the Rust programming language. They are explained with easy-to-understand examples.

2. [Design Patterns](./patterns/intro.md) section demonstrates several design patterns with minimalistic but complete projects.

3. [Anti Patterns](./anti-patterns/intro.md) section explains various non-idiomatic usages that needs to be avoided. 

4. [Algorithms](./algorithms/intro.md) section explains various popular algorithms with code samples in Rust.

The project follows a minimalistic approach, where almost all pages are short with simple, easy-to-follow code samples. Most samples can run. External documentation is referenced when necessary for extended information.

---

**If you find this project useful, please star ⭐ it on GitHub!** [![GitHub stars](https://img.shields.io/github/stars/saltukalakus/idiomatic-rust-snippets?style=social)](https://github.com/saltukalakus/idiomatic-rust-snippets/stargazers)

Your support helps keep the project active and motivates continued development.

**Contributions are welcome!** If you'd like to improve existing content, add new examples, or fix issues, feel free to open a [pull request](https://github.com/saltukalakus/idiomatic-rust-snippets/pulls). All contributions are appreciated and help make this resource better for everyone.

---

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