### Crates in Rust

In Rust, a crate is the smallest unit of code distribution. Crates can produce an executable or a library. There are two types of crates:

1. **Binary Crates**: These are programs that can be compiled to an executable.
2. **Library Crates**: These are collections of code that can be shared and reused by other programs.

### Creating a Binary Crate

To create a binary crate, you can use the `cargo new` command:

```sh
cargo new my_binary_crate
```

This will create a new directory named `my_binary_crate` with the following structure:

```
my_binary_crate/
├── Cargo.toml
└── src
    └── main.rs
```

The `main.rs` file is the entry point of the binary crate.

```rust, editable
fn main() {
    println!("Hello, world!");
}
```

### Creating a Library Crate

To create a library crate, you can use the `cargo new` command with the `--lib` flag:

```sh
cargo new my_library_crate --lib
```

This will create a new directory named `my_library_crate` with the following structure:

```
my_library_crate/
├── Cargo.toml
└── src
    └── lib.rs
```

The `lib.rs` file is the entry point of the library crate.

```rust,noplaypen
pub fn hello() {
    println!("Hello, library!");
}
```

### Using Crates

To use an external crate in your project, you need to add it to your `Cargo.toml` file. For example, to use the `rand` crate, you would add the following line to your `Cargo.toml`:

```toml
[dependencies]
rand = "0.8"
```

Then, you can use the crate in your code:

```rust,noplaypen
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let n: u32 = rng.gen();
    println!("Random number: {}", n);
}
```

Crates are a fundamental concept in Rust that allow you to organize and share your code. By understanding how to create and use crates, you can take full advantage of Rust's powerful module system.