### What is the purpose of ![no_std]?

The #![no_std] attribute in Rust is used to indicate that a crate does not use the Rust standard library (std). Instead, it relies on the core library (core), which is a subset of the standard library and is available in environments where the full standard library is not available, such as embedded systems or operating system kernels.

### Purpose of #![no_std]

**Embedded Systems**: Many embedded systems do not have the resources to support the full standard library.<br/>
**Operating System Development**: When writing an operating system, you often need to avoid dependencies on the standard library.<br/>
**Custom Environments**: Any environment where the standard library is not available or not desired.<br/>

Here’s an example of how you might use #![no_std] in a Rust crate:

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

### Key Points

**No Standard Library**: The crate does not link to the standard library.
**Use core**: You can use the core library, which provides essential functionalities like Option, Result, and basic traits.
**Limited Functionality**: Without the standard library, you lose access to many features like heap allocation, file I/O, and threading.

### Common Use Cases

**Embedded Development**: Writing firmware for microcontrollers.
**Operating System Kernels**: Developing low-level system software.
**WASM (WebAssembly)**: Targeting WebAssembly where the standard library is not fully supported.