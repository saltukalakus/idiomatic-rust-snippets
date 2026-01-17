# [Idiomatic Rust Snippets](https://idiomatic-rust-snippets.org/)

<a href="https://idiomatic-rust-snippets.org/">
<img width="1156" height="630" alt="Screenshot 2026-01-12 at 13 08 07" src="https://github.com/user-attachments/assets/d3e03730-8d11-4590-8d9a-82e48213c5af" />
</a>


Rust is known to have a steep learning curve. This book is intended to be beginner-friendly and get you up to speed with the core Rust concepts and is one of the best "cheat sheets" for Rust developers. 

1. [Essentials](./src/essentials/intro.md) covers the basic aspects of the Rust programming language. They are explained with easy to understand examples.

2. [Anti Patterns](./src/anti-patterns/intro.md) section explains various non-idiomatic usages that need to be avoided. This is the best section to read for developers who have started building projects with Rust.

3. [Design Patterns](./src/patterns/intro.md) section demonstrates several design patterns with minimalistic but complete projects. This section is for more advanced developers and design architects.

4. [Algorithms](./src/algorithms/intro.md) section explains various popular algorithms with code samples in Rust.

The project follows a minimalistic approach, where almost all pages are short with simple, easy-to-follow code samples. Most samples can run. For those samples that allow execution you can tweak the code on the online editor and test your ideas live. 

If you find this project useful, please consider giving a ⭐STAR⭐ on GitHub! If you'd like to improve existing content, add new examples, or fix issues, feel free to open a [pull request](https://github.com/saltukalakus/idiomatic-rust-snippets/pulls). All contributions are welcome.

**This project is not associated with the [Rust Foundation](https://foundation.rust-lang.org/).**

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
- `make build` - Build the book (English and Chinese)
- `make build-zh` - Build only Chinese translation
- `make serve` - Build and serve the book locally
- `make clean` - Clean build artifacts
- `make all` - Check Rust, install dependencies and build

### Multilingual Support

The book is available in multiple languages:
- **English** (default): Accessible at the root path `/`
- **简体中文** (Simplified Chinese): Accessible at `/zh-CN/`

A language selector dropdown is available in the header for easy switching between languages.

To build the Chinese translation separately:
```
./scripts/build_zh.sh
# or
make build-zh
```

The `make build` command automatically builds both English and Chinese versions.

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
 
