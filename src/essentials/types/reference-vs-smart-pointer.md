### References vs Smart Pointers

Both references and smart pointers allow you to access data indirectly, but they have important differences in ownership, memory management, and use cases.

### References (&T, &mut T)

References are the most basic form of indirection in Rust. They borrow data without taking ownership.

**Characteristics**:
- Zero runtime cost (just pointers)
- Do not own the data they point to
- Must follow Rust's borrowing rules at compile time
- Cannot outlive the data they reference
- No heap allocation involved

```rust, editable
fn main() {
    let x = 5;
    let y = &x;  // Immutable reference
    println!("x: {}, y: {}", x, y);
    
    let mut z = 10;
    let w = &mut z;  // Mutable reference
    *w += 5;
    println!("z: {}", z);
}
```

### Smart Pointers (Box, Rc, Arc, etc.)

Smart pointers are data structures that act like pointers but have additional metadata and capabilities. They own the data they point to.

**Characteristics**:
- Own the data they point to
- Automatically clean up when they go out of scope (RAII)
- May involve heap allocation
- Can have runtime overhead (reference counting, atomic operations)
- Enable patterns not possible with references alone

```rust, editable
use std::rc::Rc;

fn main() {
    // Box: Single ownership, heap allocation
    let b = Box::new(5);
    println!("Box: {}", b);
    
    // Rc: Shared ownership (single-threaded)
    let rc1 = Rc::new(10);
    let rc2 = Rc::clone(&rc1);
    println!("Rc: {}, {}", rc1, rc2);
}
```

### Key Differences

| Feature | References | Smart Pointers |
|---------|-----------|----------------|
| **Ownership** | Borrow only | Own the data |
| **Lifetime** | Tied to borrowed data | Independent |
| **Heap Allocation** | No | Usually yes |
| **Runtime Cost** | Zero | May have overhead |
| **Sharing** | Limited by borrowing rules | Rc/Arc allow sharing |
| **When to use** | Most common case | Recursive types, shared ownership, heap allocation |

### When to Use Each

**Use References when**:
- You just need to read or modify data temporarily
- The data owner is clear and the lifetime is obvious
- Performance is critical (zero-cost abstraction)

**Use Smart Pointers when**:
- You need heap allocation (`Box`)
- You need shared ownership (`Rc` for single-thread, `Arc` for multi-thread)
- You have recursive data structures (`Box`)
- You need interior mutability with shared ownership (`Rc<RefCell<T>>`)