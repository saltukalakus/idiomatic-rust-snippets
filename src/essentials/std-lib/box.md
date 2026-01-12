### Box Type

`Box<T>` is the most straightforward smart pointer in Rust. It provides heap allocation for values and enables storing data of unknown size at compile time. A `Box` owns the data it points to and deallocates it automatically when the `Box` goes out of scope.

### Basic Usage

```rust
fn main() {
    // Store an integer on the heap
    let boxed_int = Box::new(5);
    
    println!("Boxed value: {}", boxed_int);
    
    // Automatically deallocated when boxed_int goes out of scope
}
```

### Why Use Box?

**1. Large Data**: Move large structs to the heap to avoid stack overflow

```rust,ignore
struct LargeStruct {
    data: [u8; 1000000], // 1MB of data
}

fn main() {
    // This could overflow the stack
    // let large = LargeStruct { data: [0; 1000000] };
    
    // This allocates on the heap - safe
    let large = Box::new(LargeStruct { data: [0; 1000000] });
}
```

**2. Trait Objects**: Enable dynamic dispatch with traits

```rust
trait Animal {
    fn make_sound(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn make_sound(&self) {
        println!("Woof!");
    }
}

impl Animal for Cat {
    fn make_sound(&self) {
        println!("Meow!");
    }
}

fn main() {
    // Store different types that implement Animal
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog),
        Box::new(Cat),
    ];
    
    for animal in animals {
        animal.make_sound();
    }
}
```

**3. Recursive Types**: Enable recursive data structures

```rust,ignore
// This wouldn't compile without Box (infinite size)
// struct List {
//     value: i32,
//     next: List,  // Error: recursive type has infinite size
// }

// Box makes it work
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let list = Cons(1, 
        Box::new(Cons(2, 
            Box::new(Cons(3, 
                Box::new(Nil))))));
}
```

### Memory Layout

```rust
fn main() {
    let x = 5;           // Stack: 4 bytes
    let boxed = Box::new(5); // Stack: 8 bytes (pointer), Heap: 4 bytes (value)
    
    println!("Stack size of Box: {}", std::mem::size_of::<Box<i32>>());
    println!("Stack size of i32: {}", std::mem::size_of::<i32>());
}
```

### Dereferencing

```rust
fn main() {
    let boxed = Box::new(5);
    
    // Automatic dereferencing
    println!("Value: {}", boxed);
    
    // Explicit dereferencing
    println!("Value: {}", *boxed);
    
    // Modify through dereference
    let mut boxed_mut = Box::new(10);
    *boxed_mut += 5;
    println!("Modified: {}", boxed_mut);
}
```

### Ownership and Moves

```rust
fn consume_box(boxed: Box<i32>) {
    println!("Consumed: {}", boxed);
    // Box is dropped here, heap memory is freed
}

fn main() {
    let boxed = Box::new(5);
    
    consume_box(boxed);
    // boxed is no longer valid here - ownership was moved
    
    // println!("{}", boxed); // Error: value borrowed after move
}
```

### Box vs Stack Allocation

```rust
fn create_on_stack() -> i32 {
    42 // Returns a copy
}

fn create_on_heap() -> Box<i32> {
    Box::new(42) // Returns ownership of heap allocation
}

fn main() {
    let stack_val = create_on_stack();
    let heap_val = create_on_heap();
    
    println!("Stack: {}, Heap: {}", stack_val, *heap_val);
}
```

### Box with Complex Types

```rust
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let person = Box::new(Person {
        name: String::from("Alice"),
        age: 30,
    });
    
    // Access fields directly (automatic deref)
    println!("{} is {} years old", person.name, person.age);
}
```

### Converting from Box

```rust
fn main() {
    let boxed = Box::new(5);
    
    // Move value out of box (consumes the box)
    let unboxed = *boxed;
    println!("Unboxed: {}", unboxed);
    
    // boxed is no longer valid here
}
```

### Box::leak - Creating Static References

```rust
fn main() {
    let boxed = Box::new(String::from("leaked"));
    
    // Leak the box, creating a 'static reference
    let static_ref: &'static str = Box::leak(boxed);
    
    println!("Leaked string: {}", static_ref);
    // Memory is never freed (intentional leak)
}
```

### Box with Sized and Unsized Types

```rust
fn main() {
    // Box can hold unsized types like trait objects
    let numbers: Box<[i32]> = Box::new([1, 2, 3, 4, 5]);
    
    println!("Slice: {:?}", numbers);
    
    // Box with string slice
    let text: Box<str> = "hello".to_string().into_boxed_str();
    println!("Text: {}", text);
}
```

### Box in Data Structures

```rust
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
    
    fn insert_left(&mut self, value: i32) {
        self.left = Some(Box::new(Node::new(value)));
    }
    
    fn insert_right(&mut self, value: i32) {
        self.right = Some(Box::new(Node::new(value)));
    }
}

fn main() {
    let mut root = Node::new(1);
    root.insert_left(2);
    root.insert_right(3);
    
    println!("Root: {}", root.value);
}
```

### Box::from_raw and Box::into_raw

For FFI and low-level control:

```rust
fn main() {
    let boxed = Box::new(42);
    
    // Convert to raw pointer (leaks, manual management needed)
    let raw = Box::into_raw(boxed);
    
    // Use raw pointer
    unsafe {
        println!("Value: {}", *raw);
        
        // Convert back to Box (now it will be properly dropped)
        let boxed_again = Box::from_raw(raw);
    }
}
```

### Performance Considerations

```rust,ignore
fn main() {
    // Single heap allocation
    let boxed = Box::new(100);
    
    // Copying a Box is cheap (just copies the pointer)
    let ptr1 = &boxed;
    let ptr2 = &boxed;
    
    // But moving the Box transfers ownership (no copying)
    let moved = boxed;
    // boxed is no longer valid
}
```

### When to Use Box

**Use Box when:**
- Allocating large data that might overflow the stack
- Creating trait objects for dynamic dispatch
- Building recursive data structures
- Transferring ownership of heap-allocated data
- Type size must be known at compile time but content size varies

**Don't use Box when:**
- Stack allocation is sufficient (most cases)
- You need shared ownership (use `Rc` or `Arc`)
- You need interior mutability (use `RefCell` with Box if needed)

### Best Practices

- **Use Box for recursive types** - enables linked lists, trees, etc.
- **Use Box for trait objects** - enables runtime polymorphism
- **Avoid over-boxing** - stack allocation is faster for small data
- **Box is zero-cost abstraction** - no runtime overhead beyond heap allocation
- **Automatic cleanup** - no need for manual deallocation
- **Consider alternatives** - `Rc`, `Arc` for shared ownership; `Vec` for dynamic arrays
