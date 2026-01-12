### PhantomData

`PhantomData<T>` is a zero-sized type that acts as a marker to tell the compiler that your type acts "as if" it owns or uses a value of type `T`, even when it doesn't actually store it. This is crucial for implementing type-safe state machines, variance control, and ensuring proper lifetime relationships.

### What is PhantomData?

`PhantomData` is defined in the standard library as:

```rust,ignore
use std::marker::PhantomData;

pub struct PhantomData<T: ?Sized>;
```

It has no runtime representation (zero size) but affects compile-time type checking.

### Basic Usage

```rust
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

### Type State Pattern

`PhantomData` is commonly used to implement compile-time state machines:

```rust
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

### Lifetime Tracking

`PhantomData` can track lifetimes without storing references:

```rust
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

### Unused Type Parameters

When you have a generic type parameter that isn't directly used in fields:

```rust
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
        format!("{{ \"data\": \"{}\" }}", self.buffer)
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

### Variance Control

`PhantomData` affects how types are variant over their parameters:

```rust,ignore
use std::marker::PhantomData;

// Covariant (can use a more specific lifetime)
struct Covariant<'a, T> {
    _marker: PhantomData<&'a T>,
}

// Contravariant (can use a less specific lifetime)
struct Contravariant<'a, T> {
    _marker: PhantomData<fn(T) -> &'a ()>,
}

// Invariant (must match exactly)
struct Invariant<'a, T> {
    _marker: PhantomData<&'a mut T>,
}
```

### Drop Check

`PhantomData` can signal to the compiler about drop safety:

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

### Builder Pattern with Type States

```rust
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
    
    println!("Person: {} ({})", person.name, person.age);
    
    // This won't compile - must set age before building
    // let incomplete = Builder::new().name("Bob".to_string()).build();
}
```

### Protocol States

Implementing compile-time protocol validation:

```rust
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
        println!("Connecting to {}", self.address);
        Connection {
            address: self.address,
            _state: PhantomData,
        }
    }
}

impl Connection<Connecting> {
    fn finish_connect(self) -> Connection<Connected> {
        println!("Connected!");
        Connection {
            address: self.address,
            _state: PhantomData,
        }
    }
}

impl Connection<Connected> {
    fn send(&self, data: &str) {
        println!("Sending: {}", data);
    }
    
    fn disconnect(self) -> Connection<Disconnected> {
        println!("Disconnecting");
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
    conn.send("Hello");
    let _conn = conn.disconnect();
    
    // This won't compile - can't send while connecting
    // let conn = Connection::new("127.0.0.1".to_string()).connect();
    // conn.send("data"); // Error!
}
```

### Unit Types

Sometimes you can use unit types instead of `PhantomData`:

```rust,ignore
// Using unit type
struct Marker<T>(T);

fn main() {
    let _m: Marker<()> = Marker(());
}

// But PhantomData is more idiomatic for type-level markers
use std::marker::PhantomData;

struct BetterMarker<T> {
    _marker: PhantomData<T>,
}
```

### Send and Sync

`PhantomData` affects whether your type is `Send` or `Sync`:

```rust,ignore
use std::marker::PhantomData;

// Not Send (contains *const u8)
struct NotSend {
    _marker: PhantomData<*const u8>,
}

// Send (contains i32)
struct IsSend {
    _marker: PhantomData<i32>,
}
```

### Best Practices

- **Use `PhantomData` for type-state patterns** - enforce correct usage at compile time
- **Name phantom fields with underscore prefix** - `_marker`, `_phantom`, `_state`
- **Zero runtime cost** - PhantomData has no memory or performance overhead
- **Improves type safety** - catches errors at compile time instead of runtime
- **Document the purpose** - explain what the phantom type represents
- **Consider alternatives** - sometimes newtypes or associated types are clearer
- **Use for variance control** - when you need specific variance behavior
- **Essential for unsafe code** - helps ensure proper Drop and lifetime semantics
