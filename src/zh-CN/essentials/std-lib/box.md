### Box 类型

`Box<T>` 是 Rust 中最简单的智能指针，为值提供堆分配能力，允许在编译时无法确定大小的数据存储在堆上。`Box` 拥有它所指向的数据，并在 `Box` 超出作用域时自动释放内存。

### 基本用法

```rust, editable
fn main() {
    // 在堆上存储整数
    let boxed_int = Box::new(5);
    
    println!("装箱的值: {}", boxed_int);
    
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
//     next: List,  // 错误：递归类型具有无限大小
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
    let x = 5;           // 栈: 4 字节
    let boxed = Box::new(5); // 栈: 8 字节 (指针), 堆: 4 字节 (值)
    
    println!("Box 的栈大小: {}", std::mem::size_of::<Box<i32>>());
    println!("i32 的栈大小: {}", std::mem::size_of::<i32>());
}
```

### 解引用（Dereferencing）

```rust, editable
fn main() {
    let boxed = Box::new(5);
    
    // 自动解引用
    println!("值: {}", boxed);
    
    // 显式解引用
    println!("值: {}", *boxed);
    
    // 通过解引用修改值
    let mut boxed_mut = Box::new(10);
    *boxed_mut += 5;
    println!("修改后的值: {}", boxed_mut);
}
```

### 所有权与移动

```rust, editable
fn consume_box(boxed: Box<i32>) {
    println!("消费: {}", boxed);
    // Box 在此被 drop，堆内存被释放
}

fn main() {
    let boxed = Box::new(5);
    
    consume_box(boxed);
    // boxed 在此不再有效 - 所有权被移动
    
    // println!("{}", boxed); // 错误：值在移动后被借用
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
    
    println!("栈: {}, 堆: {}", stack_val, *heap_val);
}
```

### 复杂类型的 Box

```rust, editable
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let person = Box::new(Person {
        name: String::from("Alice"),
        age: 30,
    });
    
    // 直接访问字段（自动解引用）
    println!("{} 今年 {} 岁", person.name, person.age);
}
```

### 从 Box 转换

```rust, editable
fn main() {
    let boxed = Box::new(5);
    
    // 将值移出 box（消费 box）
    let unboxed = *boxed;
    println!("未装箱: {}", unboxed);
    
    // boxed 在此不再有效
}
```

### Box::leak - 创建静态引用

```rust, editable
fn main() {
    let boxed = Box::new(String::from("leaked"));
    
    // 泄漏 box，创建一个 'static 引用
    let static_ref: &'static str = Box::leak(boxed);
    
    println!("泄漏的字符串: {}", static_ref);
    // 内存永远不会被释放（有意泄漏）
}
```

### Box 与 Sized 和 Unsized 类型

```rust, editable
fn main() {
    // Box 可以持有 unsized 类型，如特质对象
    let numbers: Box<[i32]> = Box::new([1, 2, 3, 4, 5]);
    
    println!("切片: {:?}", numbers);
    
    // Box 与字符串切片
    let text: Box<str> = "hello".to_string().into_boxed_str();
    println!("文本: {}", text);
}
```

### 数据结构中的 Box

```rust, editable
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
    
    println!("根: {}", root.value);
}
```

### Box::from_raw 和 Box::into_raw

用于 FFI 和底层控制：

```rust, editable
fn main() {
    let boxed = Box::new(42);
    
    // 转换为裸指针（泄漏，需要手动管理）
    let raw = Box::into_raw(boxed);
    
    // 使用裸指针
    unsafe {
        println!("值: {}", *raw);
        
        // 转换回 Box（现在它将被正确地 drop）
        let boxed_again = Box::from_raw(raw);
    }
}
```

### 性能考虑

```rust,ignore
fn main() {
    // 单次堆分配
    let boxed = Box::new(100);
    
    // 复制 Box 很廉价（只复制指针）
    let ptr1 = &boxed;
    let ptr2 = &boxed;
    
    // 但移动 Box 会转移所有权（不复制）
    let moved = boxed;
    // boxed 不再有效
}
```

### 何时使用 Box

**在以下情况使用 Box：**
- 分配可能溢出栈的大数据
- 为动态分发创建特质对象
- 构建递归数据结构
- 转移堆分配数据的所有权
- 编译时类型大小必须已知，但内容大小可变

**不建议使用 Box 的情况：**
- 栈分配已足够（大多数情况）
- 需要共享所有权（使用 `Rc` 或 `Arc`）
- 需要内部可变性（必要时可以与 `RefCell` 一起使用）

### 最佳实践

- **为递归类型使用 Box** - 支持链表、树等结构
- **为特质对象使用 Box** - 支持运行时多态
- **避免过度使用 Box** - 对于小数据，栈分配更快
- **Box 是零开销抽象** - 除了堆分配之外没有运行时开销
- **自动清理** - 无需手动释放
- **考虑替代方案** - `Rc`、`Arc` 用于共享所有权；`Vec` 用于动态数组
