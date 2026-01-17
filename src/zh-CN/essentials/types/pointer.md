### 指针及其在 Rust 中的使用

在 Rust 中，指针是表示内存地址的类型。Rust 提供了多种指针类型，每种都有不同的特性和使用场景。指针用于引用内存中存储的数据，而 Rust 的所有权与借用规则保证指针被安全使用。

### Rust 中的指针类型

**引用（References）**：不可变与可变引用（`&T` 与 `&mut T`）。<br/>
**智能指针（Smart Pointers）**：`Box`、`Rc`、`Arc` 等。<br/>
**裸指针（Raw Pointers）**：不安全指针，用于底层内存操作。<br/>

### 引用

引用是 Rust 中最常见的指针类型，允许在不取得所有权的情况下借用数据。

不可变引用（`&T`）：允许读取数据但不能修改。<br/>
可变引用（`&mut T`）：允许修改数据。<br/>

```rust, editable
fn main() {
    let x = 5;
    let y = &x; // 不可变引用
    println!("y: {}", y);

    let mut z = 10;
    let w = &mut z; // 可变引用
    *w += 5;
    println!("z: {}", z);
}
```

### 智能指针

智能指针不仅表现得像指针，还具有额外功能，例如自动内存管理。Rust 常见的智能指针包括：

**Box**：在堆上分配的指针。<br/>

```rust, editable
fn main() {
    let b = Box::new(5);
    println!("b: {}", b);
}
```

**Rc**：适用于单线程场景的引用计数指针。<br/>

```rust, editable
use std::rc::Rc;

fn main() {
    let a = Rc::new(5);
    let b = Rc::clone(&a);
    println!("a: {}, b: {}", a, b);
}
```

**Arc**：适用于多线程场景的原子引用计数指针。<br/>

```rust, editable
use std::sync::Arc;
use std::thread;

fn main() {
    let a = Arc::new(5);
    let b = Arc::clone(&a);

    let handle = thread::spawn(move || {
        println!("b: {}", b);
    });

    println!("a: {}", a);
    handle.join().unwrap();
}
```

### 裸指针

裸指针是不安全的指针，可用于执行低级内存操作。它们不受 Rust 借用规则的约束，必须在 `unsafe` 块中使用。

不可变裸指针（`*const T`）：指向不可修改的数据。<br/>
可变裸指针（`*mut T`）：指向可修改的数据。<br/>

```rust, editable
fn main() {
    let x = 5;
    let y = &x as *const i32; // 不可变裸指针

    let mut z = 10;
    let w = &mut z as *mut i32; // 可变裸指针

    unsafe {
        println!("y: {}", *y);
        *w += 5;
        println!("z: {}", *w);
    }
}
```
