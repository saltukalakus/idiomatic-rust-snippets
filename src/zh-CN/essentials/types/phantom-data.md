### PhantomData

`PhantomData<T>` 是一个零大小的类型，它充当一个标记，告诉编译器你的类型“好像”拥有或使用一个 T 类型的值，即使它实际上并没有存储它。这对于实现类型安全的状态机、方差控制和确保正确的生命周期关系至关重要。

### 什么是 PhantomData？

`PhantomData` 在标准库中定义为：

```rust,ignore
use std::marker::PhantomData;

pub struct PhantomData<T: ?Sized>;
```

它没有运行时表示（零大小），但会影响编译时类型检查。

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
    println!("值: {}", wrapper.value);
    
    // 大小就是 i32 的大小，PhantomData 是零大小的
    println!("大小: {}", std::mem::size_of::<Wrapper<String>>());
}
```

### 类型状态模式

`PhantomData` 通常用于实现编译时状态机：

```rust, editable
use std::marker::PhantomData;

// 状态类型
struct Locked;
struct Unlocked;

struct Door<State> {
    _state: PhantomData<State>,
}

impl Door<Locked> {
    fn new() -> Self {
        println!("门是锁着的");
        Door { _state: PhantomData }
    }
    
    fn unlock(self) -> Door<Unlocked> {
        println!("正在开锁");
        Door { _state: PhantomData }
    }
}

impl Door<Unlocked> {
    fn lock(self) -> Door<Locked> {
        println!("正在上锁");
        Door { _state: PhantomData }
    }
    
    fn open(&self) {
        println!("门开了！");
    }
}

fn main() {
    let door = Door::<Locked>::new();
    let door = door.unlock();
    door.open();
    let door = door.lock();
    
    // 这不会编译 - 不能打开锁着的门！
    // door.open(); // 错误: 在 `Door<Locked>` 类型上找不到名为 `open` 的方法
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
    println!("处理上下文 {}", ctx.id);
}

fn main() {
    let ctx = Context::new(1);
    process(ctx);
}
```

### 未使用的类型参数

当你的泛型类型参数没有在字段中直接使用时：

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

// 为 Json 特化
impl Serializer<Json> {
    fn to_json(&self) -> String {
        format!("{{ \"data\": \"{}\" }}", self.buffer)
    }
}

// 为 Xml 特化
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

### 方差控制

`PhantomData` 会影响你的类型在其参数上的方差：

```rust,ignore
use std::marker::PhantomData;

// 协变 (可以使用更具体的生命周期)
struct Covariant<'a, T> {
    _marker: PhantomData<&'a T>,
}

// 逆变 (可以使用更不具体的生命周期)
struct Contravariant<'a, T> {
    _marker: PhantomData<fn(T) -> &'a ()>,
}

// 不变 (必须完全匹配)
struct Invariant<'a, T> {
    _marker: PhantomData<&'a mut T>,
}
```

### Drop 检查

`PhantomData` 可以向编译器发出关于 drop 安全性的信号：

```rust,ignore
use std::marker::PhantomData;

struct Inspector<T> {
    name: String,
    _owns_t: PhantomData<T>,
}

impl<T> Drop for Inspector<T> {
    fn drop(&mut self) {
        println!("正在 drop inspector: {}", self.name);
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

### 使用类型状态的构建器模式

```rust, editable
use std::marker::PhantomData;

struct Incomplete;
struct Complete;

struct Builder<State> {
    name: Option<String>,
    age: Option<u32>,
    _state: PhantomData<State>,
}

impl Builder<Incomplete> {
    fn new() -> Self {
        Builder {
            name: None,
            age: None,
            _state: PhantomData,
        }
    }
    
    fn name(mut self, name: String) -> Builder<Incomplete> {
        self.name = Some(name);
        self
    }
    
    fn age(mut self, age: u32) -> Builder<Complete> {
        self.age = Some(age);
        Builder {
            name: self.name,
            age: Some(age),
            _state: PhantomData,
        }
    }
}

impl Builder<Complete> {
    fn build(self) -> Person {
        Person {
            name: self.name.unwrap(),
            age: self.age.unwrap(),
        }
    }
}

struct Person {
    name: String,
    age: u32,
}

fn main() {
    let person = Builder::new()
        .name("Alice".to_string())
        .age(30)
        .build();
    
    println!("人: {} ({})", person.name, person.age);
    
    // 这不会编译 - 必须在构建前设置年龄
    // let incomplete = Builder::new().name("Bob".to_string()).build();
}
```

### 协议状态

实现编译时协议验证：

```rust, editable
use std::marker::PhantomData;

struct Connecting;
struct Connected;
struct Disconnected;

struct Connection<State> {
    address: String,
    _state: PhantomData<State>,
}

impl Connection<Disconnected> {
    fn new(address: String) -> Self {
        Connection {
            address,
            _state: PhantomData,
        }
    }
    
    fn connect(self) -> Connection<Connecting> {
        println!("正在连接到 {}", self.address);
        Connection {
            address: self.address,
            _state: PhantomData,
        }
    }
}

impl Connection<Connecting> {
    fn finish_connect(self) -> Connection<Connected> {
        println!("已连接！");
        Connection {
            address: self.address,
            _state: PhantomData,
        }
    }
}

impl Connection<Connected> {
    fn send(&self, data: &str) {
        println!("正在发送: {}", data);
    }
    
    fn disconnect(self) -> Connection<Disconnected> {
        println!("正在断开连接");
        Connection {
            address: self.address,
            _state: PhantomData,
        }
    }
}

fn main() {
    let conn = Connection::new("127.0.0.1".to_string());
    let conn = conn.connect();
    let conn = conn.finish_connect();
    conn.send("你好");
    let _conn = conn.disconnect();
    
    // 这不会编译 - 不能在连接时发送
    // let conn = Connection::new("127.0.0.1".to_string()).connect();
    // conn.send("data"); // 错误！
}
```

### 单元类型

有时你可以使用单元类型而不是 `PhantomData`：

```rust,ignore
// 使用单元类型
struct Marker<T>(T);

fn main() {
    let _m: Marker<()> = Marker(());
}

// 但对于类型级标记，PhantomData 更符合习惯
use std::marker::PhantomData;

struct BetterMarker<T> {
    _marker: PhantomData<T>,
}
```

### Send 和 Sync

`PhantomData` 会影响你的类型是否是 `Send` 或 `Sync`：

```rust,ignore
use std::marker::PhantomData;

// 不是 Send (包含 *const u8)
struct NotSend {
    _marker: PhantomData<*const u8>,
}

// 是 Send (包含 i32)
struct IsSend {
    _marker: PhantomData<i32>,
}
```

### 最佳实践

- **对类型状态模式使用 `PhantomData`** - 在编译时强制正确使用
- **用下划线前缀命名 phantom 字段** - `_marker`, `_phantom`, `_state`
- **零运行时成本** - PhantomData 没有内存或性能开销
- **提高类型安全** - 在编译时而不是运行时捕获错误
- **记录目的** - 解释 phantom 类型代表什么
- **考虑替代方案** - 有时 newtypes 或关联类型更清晰
- **用于方差控制** - 当你需要特定的方差行为时
- **对于不安全代码至关重要** - 帮助确保正确的 Drop 和生命周期语义
