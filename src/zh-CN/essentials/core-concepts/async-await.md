````markdown
### 异步 / await

Rust 中的异步编程允许你编写在等待 I/O 操作时不阻塞线程的并发代码。`async`/`await` 语法使异步代码看起来和行为类似同步代码，从而更易读写。

**注意**：以下示例需要像 Tokio 这样的异步运行时。将其添加到你的 `Cargo.toml`：
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

### 什么是 Async/Await？

`async` 关键字将函数转换为返回 `Future` 的异步函数。`await` 关键字在等待 `Future` 完成时暂停异步函数的执行，但不会阻塞线程。

```rust,ignore
async fn fetch_data() -> String {
    // 异步函数
    "data".to_string()
}

async fn process_data() {
    let data = fetch_data().await; // 在此处等待
    println!("Got: {}", data);
}
```

### Future 特征

Rust 的异步系统的核心是 `Future` 特征。将函数标记为 `async` 时，它返回一个实现了 `Future` 的类型：

```rust,ignore
use std::future::Future;

// 一个异步函数
async fn example() -> i32 {
    42
}

// 大致等价于：
fn example_desugared() -> impl Future<Output = i32> {
    async { 42 }
}
```

### 与 Tokio 的基本用法

标准库提供了 async/await 语法，但需要像 Tokio 或 async-std 这样的运行时来执行异步代码：

```rust,ignore
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("Starting...");
    sleep(Duration::from_secs(1)).await;
    println!("Done!");
}
```

### 并发运行多个任务

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
    // 并发运行任务
    tokio::join!(task_one(), task_two());
    // 两个任务大约在 1 秒内完成，而不是 2 秒！
}
```

### 异步 vs 线程

**Async/Await**:
- 轻量（单线程上可有成千上万的任务）
- 适用于 I/O 密集型操作
- 默认单线程（运行时可配置为多线程）
- 切换开销低
- 任务协作式让出执行

**线程**:
- 更“重”，每进程的数量有限
- 适用于 CPU 密集型操作
- 在多核上能实现真正并行
- 切换开销更高
- 由操作系统调度

### 错误处理

在异步代码中像同步代码那样使用 `Result` 和 `?`：

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
```

### 生成并发任务与等待

```rust,ignore
use tokio::task;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let handle = task::spawn(async {
        sleep(Duration::from_secs(1)).await;
        "task result"
    });
    let result = handle.await.unwrap();
    println!("Task returned: {}", result);
}
```

### 常见运行时

**Tokio**：最流行、功能丰富、适合网络服务
**async-std**：API 类似标准库，更简单
**smol**：小巧高效
**embassy**：用于嵌入式系统

### 最佳实践

- **对 I/O 密集型工作使用 async**
- **对 CPU 密集型工作使用线程**
- **不要在异步代码中阻塞**，避免使用 `thread::sleep`
- **注意在循环中使用 `.await`**，考虑使用 `join!` 或 `select!`
- **为用例选择合适的运行时**
- **显式处理错误**——Future 只有在被 await 时才会执行
- **在异步上下文中用 `tokio::spawn` 获取并发**

````
