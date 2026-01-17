### PhantomData

`PhantomData<T>` 是一个零大小类型，用作标记，告诉编译器你的类型“好像”拥有或使用类型 `T` 的值，即使它实际上并不存储该值。这在实现类型安全的状态机、控制协变/逆变以及确保正确的生命周期关系时非常重要。

### 什么是 PhantomData？

`PhantomData` 在标准库中定义为：

```rust,ignore
use std::marker::PhantomData;

pub struct PhantomData<T: ?Sized>;
```

它在运行时没有表示（零大小），但会影响编译时的类型检查。

### 基本用法

```rust, editable
use std::marker::PhantomData;

struct Wrapper<T> {
    _marker: PhantomData<T>,
    value: i32,
}

impl<T> Wrapper<T> {
    fn new(value: i32) -> Self {
        Wrapper {
            _marker: PhantomData,
            value,
        }
    }
}

fn main() {
    let wrapper: Wrapper<String> = Wrapper::new(42);
    println!("Value: {}", wrapper.value);
    
    // Size is just the size of i32, PhantomData is zero-sized
    println!("Size: {}", std::mem::size_of::<Wrapper<String>>());
}
```

### 类型状态模式

`PhantomData` 常用于实现编译期状态机：

```rust, editable
use std::marker::PhantomData;

// State types
struct Locked;
struct Unlocked;

struct Door<State> {
    _state: PhantomData<State>,
}

impl Door<Locked> {
    fn new() -> Self {
        println!("Door is locked");
        Door { _state: PhantomData }
    }
    
    fn unlock(self) -> Door<Unlocked> {
        println!("Unlocking door");
        Door { _state: PhantomData }
    }
}

impl Door<Unlocked> {
    fn lock(self) -> Door<Locked> {
        println!("Locking door");
        Door { _state: PhantomData }
    }
    
    fn open(&self) {
        println!("Door opened!");
    }
}

fn main() {
    let door = Door::<Locked>::new();
    let door = door.unlock();
    door.open();
    let door = door.lock();
    
    // This won't compile - can't open a locked door!
    // door.open(); // Error: no method named `open` found for type `Door<Locked>`
}
```

### 生命周期跟踪

`PhantomData` 可以在不存储引用的情况下跟踪生命周期：

```rust, editable
use std::marker::PhantomData;

struct Context<'a> {
    _lifetime: PhantomData<&'a ()>,
    id: u32,
}

impl<'a> Context<'a> {
    fn new(id: u32) -> Self {
        Context {
            _lifetime: PhantomData,
            id,
        }
    }
}

fn process<'a>(ctx: Context<'a>) {
    println!("Processing context {}", ctx.id);
}

fn main() {
    let ctx = Context::new(1);
    process(ctx);
}
```

### 未使用的类型参数

当你的泛型类型参数没有直接在字段中使用时，可以用 `PhantomData`：

```rust, editable
use std::marker::PhantomData;

trait Format {}
struct Json;
struct Xml;

impl Format for Json {}
impl Format for Xml {}

struct Serializer<F: Format> {
    buffer: String,
    _format: PhantomData<F>,
}

impl<F: Format> Serializer<F> {
    fn new() -> Self {
        Serializer {
            buffer: String::new(),
            _format: PhantomData,
        }
    }
    
    fn add(&mut self, data: &str) {
        self.buffer.push_str(data);
    }
}

// Specialize for Json
impl Serializer<Json> {
    fn to_json(&self) -> String {
        format!("{ \"data\": \"{}\" }", self.buffer)
    }
}

// Specialize for Xml
impl Serializer<Xml> {
    fn to_xml(&self) -> String {
        format!("<data>{}</data>", self.buffer)
    }
}

fn main() {
    let mut json = Serializer::<Json>::new();
    json.add("test");
    println!("{}", json.to_json());
    
    let mut xml = Serializer::<Xml>::new();
    xml.add("test");
    println!("{}", xml.to_xml());
}
```

### 协变/逆变控制

`PhantomData` 会影响类型在其参数上的协变性：它可以用来控制类型参数如何随生命周期变化。

### Drop 检查

`PhantomData` 可以向编译器表明 drop（析构）安全性：

```rust,ignore
use std::marker::PhantomData;

struct Inspector<T> {
    name: String,
    _owns_t: PhantomData<T>,
}

impl<T> Drop for Inspector<T> {
    fn drop(&mut self) {
        println!("Dropping inspector: {}", self.name);
    }
}

impl<T> Inspector<T> {
    fn new(name: String) -> Self {
        Inspector {
            name,
            _owns_t: PhantomData,
        }
    }
}
```

### 构建器模式与类型状态

（见上方示例）

### 协议状态

实现编译时协议验证（见上方示例）。

### 单元类型

有时可使用单元类型替代 `PhantomData`，但 `PhantomData` 在类型级标记方面更常见。

### Send 与 Sync

`PhantomData` 会影响你的类型是否实现 `Send` 或 `Sync`。
