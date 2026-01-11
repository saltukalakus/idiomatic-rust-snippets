### Async/Await

Asynchronous programming in Rust allows you to write concurrent code that doesn't block threads while waiting for I/O operations. The `async`/`await` syntax makes asynchronous code look and behave like synchronous code, making it easier to read and write.

**Note**: The examples below require an async runtime like Tokio. Add to your `Cargo.toml`:
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

### What is Async/Await?

The `async` keyword transforms a function into an asynchronous function that returns a `Future`. The `await` keyword pauses execution of an async function until the awaited `Future` completes, without blocking the thread.

```rust,ignore
async fn fetch_data() -> String {
    // This is an async function
    "data".to_string()
}

async fn process_data() {
    let data = fetch_data().await; // await pauses here
    println!("Got: {}", data);
}
```

### The Future Trait

At the core of Rust's async system is the `Future` trait. When you mark a function as `async`, it returns a type that implements `Future`:

```rust,ignore
use std::future::Future;

// An async function
async fn example() -> i32 {
    42
}

// Is roughly equivalent to:
fn example_desugared() -> impl Future<Output = i32> {
    async { 42 }
}
```

### Basic Usage with Tokio

Rust's standard library provides the async/await syntax, but you need a runtime like Tokio or async-std to execute async code:

```rust,ignore
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("Starting...");
    
    // Sleep without blocking the thread
    sleep(Duration::from_secs(1)).await;
    
    println!("Done!");
}
```

### Running Multiple Tasks Concurrently

```rust,ignore
use tokio::time::{sleep, Duration};

async fn task_one() {
    sleep(Duration::from_secs(1)).await;
    println!("Task one complete");
}

async fn task_two() {
    sleep(Duration::from_secs(1)).await;
    println!("Task two complete");
}

#[tokio::main]
async fn main() {
    // Run tasks concurrently
    tokio::join!(task_one(), task_two());
    // Both complete in ~1 second, not 2!
}
```

### Async vs Threads

**Async/Await**:
- Lightweight (thousands of tasks on one thread)
- Best for I/O-bound operations
- Single-threaded by default (can be multi-threaded with runtime)
- Lower overhead for context switching
- Tasks cooperatively yield execution

**Threads**:
- Heavier weight (limited number per process)
- Best for CPU-bound operations
- True parallelism on multi-core systems
- Higher overhead for context switching
- OS handles scheduling

```rust,ignore
use tokio::time::{sleep, Duration};
use std::thread;

#[tokio::main]
async fn main() {
    // Async approach - efficient for I/O
    let async_start = std::time::Instant::now();
    tokio::join!(
        sleep(Duration::from_millis(100)),
        sleep(Duration::from_millis(100)),
        sleep(Duration::from_millis(100)),
    );
    println!("Async took: {:?}", async_start.elapsed()); // ~100ms
    
    // Thread approach - heavier
    let thread_start = std::time::Instant::now();
    let handles: Vec<_> = (0..3).map(|_| {
        thread::spawn(|| {
            thread::sleep(Duration::from_millis(100));
        })
    }).collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Threads took: {:?}", thread_start.elapsed()); // ~100ms but more overhead
}
```

### Error Handling in Async Code

Use `Result` with `.await` just like synchronous code:

```rust,ignore
use std::io;

async fn read_file() -> io::Result<String> {
    tokio::fs::read_to_string("file.txt").await
}

async fn process() -> io::Result<()> {
    let content = read_file().await?;
    println!("File content: {}", content);
    Ok(())
}

#[tokio::main]
async fn main() {
    match process().await {
        Ok(_) => println!("Success!"),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### Spawning Tasks

Create independent async tasks that run concurrently:

```rust,ignore
use tokio::task;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let handle = task::spawn(async {
        sleep(Duration::from_secs(1)).await;
        "task result"
    });
    
    println!("Task spawned, doing other work...");
    
    // Wait for the task to complete
    let result = handle.await.unwrap();
    println!("Task returned: {}", result);
}
```

### Select and Racing Futures

Execute multiple futures and act on the first one to complete:

```rust,ignore
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    tokio::select! {
        _ = sleep(Duration::from_secs(1)) => {
            println!("Timeout reached");
        }
        _ = async {
            sleep(Duration::from_millis(500)).await;
            println!("Task completed");
        } => {}
    }
}
```

### Async Traits (Limitations)

Currently, async functions in traits require workarounds or the `async-trait` crate:

```toml
# Add to Cargo.toml
[dependencies]
async-trait = "0.1"
```

```rust,ignore
use async_trait::async_trait;

#[async_trait]
trait DataFetcher {
    async fn fetch(&self) -> String;
}

struct ApiClient;

#[async_trait]
impl DataFetcher for ApiClient {
    async fn fetch(&self) -> String {
        "data from API".to_string()
    }
}
```

### Common Async Runtimes

**Tokio**: Most popular, feature-rich, good for network services
**async-std**: Standard library-like API, simpler
**smol**: Minimal and efficient
**embassy**: For embedded systems

### Best Practices

- **Use async for I/O-bound work** (network, file I/O, timers)
- **Use threads for CPU-bound work** (heavy computation)
- **Don't block in async code** - avoid `thread::sleep`, use `tokio::time::sleep`
- **Beware of `.await` in loops** - consider `join!` or `select!` for concurrency
- **Choose the right runtime** for your use case
- **Handle errors explicitly** - futures don't execute until awaited
- **Use `tokio::spawn` for true concurrency** within an async context
