### How to use unstable features in your Rust project

In Rust, the #! syntax is used for attributes that apply to the entire crate or module. These are known as "inner attributes." They are typically placed at the top of a file and are used to configure various aspects of the Rust compiler's behavior for that file or crate.

#![feature(...)]: This attribute allows you to use unstable features in your Rust code from the nightly Rust compiler. It must be placed at the top of the file. 

As an example, the #![feature(test)] attribute in Rust is used to enable the test crate, which includes benchmarking and testing utilities that are not yet stabilized.

```rust,noplaypen
#![feature(test)]
extern crate bcrypt;
extern crate test;

use bcrypt::{hash, DEFAULT_COST};

#[bench]
fn bench_cost_4(b: &mut test::Bencher) {
    b.iter(|| hash("hunter2", 4));
}
```