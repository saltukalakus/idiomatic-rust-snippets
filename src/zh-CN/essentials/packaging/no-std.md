````markdown
### #![no_std] 的用途是什么？

Rust 中的 `#![no_std]` 属性用于指示一个 crate 不使用 Rust 标准库 (std)。相反，它依赖于核心库 (core)，核心库是标准库的子集，可在没有完整标准库的环境中使用，例如嵌入式系统或操作系统内核。

用例可分为以下几类：

* 许多嵌入式系统没有支持完整标准库的资源。<br/>
* 编写操作系统时，通常需要避免对标准库的依赖。<br/>
* 任何标准库不可用或不希望使用的环境。<br/>
* 目标为 WebAssembly，其中标准库未完全支持。

```rust,ignore
#![no_std]

use core::fmt;

// 我们将使其可显示的自定义结构
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

// 在 no_std 环境中，通常提供自定义的 panic 处理程序
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// 对于嵌入式系统，入口点通常不同
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let my_struct = MyStruct::new(42);
    // 在真实的嵌入式代码中，您将在此处与硬件交互
    // 没有可用的 println! - 将使用特定于硬件的输出
    loop {}
}
```

**注意**: `no_std` crates 不能使用 `println!`、`Vec`、`String` 或其他仅限 std 的功能。它们依赖于 `core`（语言原语）和 `alloc`（如果堆分配可用）。
````
