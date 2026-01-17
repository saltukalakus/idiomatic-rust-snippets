### Box 类型

`Box<T>` 是 Rust 中最简单的智能指针，为值提供堆分配能力，允许在编译时无法确定大小的数据存储在堆上。`Box` 拥有它所指向的数据，并在 `Box` 超出作用域时自动释放内存。

### 基本用法

```rust, editable
fn main() {
    // 在堆上存储整数
    let boxed_int = Box::new(5);
    
    println!("Boxed value: {}", boxed_int);
    
    // boxed_int 超出作用域时会自动释放
}
```

### 为什么使用 Box？

**1. 大数据**：将大型结构体移动到堆上以避免栈溢出

```rust,ignore
struct LargeStruct {
    data: [u8; 1000000], // 1MB 数据
}

fn main() {
    // 这可能导致栈溢出
    // let large = LargeStruct { data: [0; 1000000] };
    
    // 这会在堆上分配 - 更安全
    let large = Box::new(LargeStruct { data: [0; 1000000] });
}
```

**2. 特质对象（Trait Objects）**：支持基于特质的动态分发

```rust, editable
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
    // 存储实现了 Animal 的不同类型
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog),
        Box::new(Cat),
    ];
    
    for animal in animals {
        animal.make_sound();
    }
}
```

**3. 递归类型**：支持递归数据结构

```rust,ignore
// 如果没有 Box，这将无法编译（无限大小）
// struct List {
//     value: i32,
//     next: List,  // Error: recursive type has infinite size
// }

// 使用 Box 可以解决
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

### 内存布局

```rust, editable
fn main() {
    let x = 5;           // Stack: 4 bytes
    let boxed = Box::new(5); // Stack: 8 bytes (pointer), Heap: 4 bytes (value)
    
    println!("Stack size of Box: {}", std::mem::size_of::<Box<i32>>());
    println!("Stack size of i32: {}", std::mem::size_of::<i32>());
}
```

### 解引用（Dereferencing）

```rust, editable
fn main() {
    let boxed = Box::new(5);
    
    // 自动解引用
    println!("Value: {}", boxed);
    
    // 显式解引用
    println!("Value: {}", *boxed);
    
    // 通过解引用修改值
    let mut boxed_mut = Box::new(10);
    *boxed_mut += 5;
    println!("Modified: {}", boxed_mut);
}
```

### 所有权与移动

```rust, editable
fn consume_box(boxed: Box<i32>) {
    println!("Consumed: {}", boxed);
    // Box 在此被 drop，堆内存被释放
}

fn main() {
    let boxed = Box::new(5);
    
    consume_box(boxed);
    // boxed 在此不再有效 - 所有权被移动
    
    // println!("{}", boxed); // Error: value borrowed after move
}
```

### Box 与栈分配

```rust, editable
fn create_on_stack() -> i32 {
    42 // 返回一个副本
}

fn create_on_heap() -> Box<i32> {
    Box::new(42) // 返回堆分配的所有权
}

fn main() {
    let stack_val = create_on_stack();
    let heap_val = create_on_heap();
    
    println!("Stack: {}, Heap: {}", stack_val, *heap_val);
}
```

### 何时使用 Box

- 在需要堆分配大数据时
- 为特质对象创建动态分发时
- 构建递归数据结构时
- 转移堆分配数据的所有权时
- 当编译期类型大小未知但内容大小可变时

### 不建议使用 Box 的情况

- 栈分配已足够（大多数情况）
- 需要共享所有权（请使用 `Rc` 或 `Arc`）
- 需要内部可变性（必要时可以与 `RefCell` 一起使用）

### 最佳实践

- **为递归类型使用 Box** - 支持链表、树等结构
- **为特质对象使用 Box** - 支持运行时多态
- **避免过度使用 Box** - 对于小数据，栈分配更快
- **Box 是零开销抽象** - 除了堆分配之外没有运行时开销
- **自动清理** - 无需手动释放
- **考虑替代方案** - 使用 `Rc`、`Arc` 进行共享所有权；使用 `Vec` 进行动态数组
