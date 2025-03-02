### What is the purpose of ![no_std]?

The #![no_std] attribute in Rust is used to indicate that a crate does not use the Rust standard library (std). Instead, it relies on the core library (core), which is a subset of the standard library and is available in environments where the full standard library is not available, such as embedded systems or operating system kernels.

Usecases can be grouped into a few items;

* Many embedded systems do not have the resources to support the full standard library.<br/>
* When writing an operating system, you often need to avoid dependencies on the standard library.<br/>
* Any environment where the standard library is not available or not desired.<br/>
* Targeting WebAssembly where the standard library is not fully supported.

```rust
#![no_std]

extern crate core;

use core::fmt;

struct MyStruct;

impl fmt::Display for MyStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Hello, no_std!")
    }
}

fn main() {
    let my_struct = MyStruct;
    println!("{}", my_struct); // This will not work because println! is part of std
}
```