### What is the purpose of #![no_std]?

The `#![no_std]` attribute in Rust is used to indicate that a crate does not use the Rust standard library (std). Instead, it relies on the core library (core), which is a subset of the standard library and is available in environments where the full standard library is not available, such as embedded systems or operating system kernels.

Usecases can be grouped into a few items:

* Many embedded systems do not have the resources to support the full standard library.<br/>
* When writing an operating system, you often need to avoid dependencies on the standard library.<br/>
* Any environment where the standard library is not available or not desired.<br/>
* Targeting WebAssembly where the standard library is not fully supported.

```rust,ignore
#![no_std]

use core::fmt;

// A custom struct that we'll make displayable
pub struct MyStruct {
    value: i32,
}

impl MyStruct {
    pub const fn new(value: i32) -> Self {
        MyStruct { value }
    }
    
    pub fn get_value(&self) -> i32 {
        self.value
    }
}

impl fmt::Display for MyStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MyStruct {{ value: {} }}", self.value)
    }
}

// In no_std environments, you typically provide custom panic handler
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// For embedded systems, entry point is often different
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let my_struct = MyStruct::new(42);
    // In real embedded code, you'd interact with hardware here
    // No println! available - would use hardware-specific output
    loop {}
}
```

**Note**: `no_std` crates cannot use `println!`, `Vec`, `String`, or other std-only features. They rely on `core` (language primitives) and `alloc` (if heap allocation is available).