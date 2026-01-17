### 异步/Await

Rust 中的异步编程允许您编写在等待 I/O 操作时不会阻塞线程的并发代码。`async`/`await` 语法使得异步代码的外观和行为都像同步代码，从而更易于读写。

**注意**: 以下示例需要像 Tokio 这样的异步运行时。请将以下内容添加到您的 `Cargo.toml` 中：
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

### 什么是 Async/Await？

`async` 关键字将一个函数转换为一个返回 `Future` 的异步函数。`await` 关键字会暂停异步函数的执行，直到被等待的 `Future` 完成，而不会阻塞线程。

```rust,ignore
async fn fetch_data() -> String {
    // 这是一个异步函数
    "data".to_string()
}

async fn process_data() {
    let data = fetch_data().await; // await 在这里暂停
    println!("获取到: {}", data);
}
```

### Future 特征

Rust 异步系统的核心是 `Future` 特征。当您将一个函数标记为 `async` 时，它会返回一个实现了 `Future` 的类型：

```rust,ignore
use std::future::Future;

// 一个异步函数
async fn example() -> i32 {
    42
}

// 大致等同于：
fn example_desugared() -> impl Future<Output = i32> {
    async { 42 }
}
```

### 使用 Tokio 的基本用法

Rust 的标准库提供了 async/await 语法，但您需要一个像 Tokio 或 async-std 这样的运行时来执行异步代码：

```rust,ignore
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("开始...");
    
    // 不阻塞线程的休眠
    sleep(Duration::from_secs(1)).await;
    
    println!("完成!");
}
```

### 并发运行多个任务

```rust,ignore
use tokio::time::{sleep, Duration};

async fn task_one() {
    sleep(Duration::from_secs(1)).await;
    println!("任务一完成");
}

async fn task_two() {
    sleep(Duration::from_secs(1)).await;
    println!("任务二完成");
}

#[tokio::main]
async fn main() {
    // 并发运行任务
    tokio::join!(task_one(), task_two());
    // 两者都在约 1 秒内完成，而不是 2 秒！
}
```

### 异步 vs 线程

**Async/Await**:
- 轻量级（一个线程上可以有成千上万个任务）
- 最适合 I/O 密集型操作
- 默认是单线程的（可以通过运行时实现多线程）
- 上下文切换的开销较低
- 任务协作式地让出执行权

**线程**:
- 重量级（每个进程的数量有限）
- 最适合 CPU 密集型操作
- 在多核系统上实现真正的并行
- 上下文切换的开销较高
- 操作系统处理调度

```rust,ignore
use tokio::time::{sleep, Duration};
use std::thread;

#[tokio::main]
async fn main() {
    // 异步方法 - 对 I/O 高效
    let async_start = std::time::Instant::now();
    tokio::join!(
        sleep(Duration::from_millis(100)),
        sleep(Duration::from_millis(100)),
        sleep(Duration::from_millis(100)),
    );
    println!("异步耗时: {:?}", async_start.elapsed()); // ~100ms
    
    // 线程方法 - 更重量级
    let thread_start = std::time::Instant::now();
    let handles: Vec<_> = (0..3).map(|_| {
        thread::spawn(|| {
            thread::sleep(Duration::from_millis(100));
        })
    }).collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
    println!("线程耗时: {:?}", thread_start.elapsed()); // ~100ms 但开销更大
}
```

### 异步代码中的错误处理

像同步代码一样，将 `Result` 与 `.await` 结合使用：

```rust,ignore
use std::io;

async fn read_file() -> io::Result<String> {
    tokio::fs::read_to_string("file.txt").await
}

async fn process() -> io::Result<()> {
    let content = read_file().await?;
    println!("文件内容: {}", content);
    Ok(())
}

#[tokio::main]
async fn main() {
    match process().await {
        Ok(_) => println!("成功!"),
        Err(e) => eprintln!("错误: {}", e),
    }
}
```

### 生成任务

创建并发运行的独立异步任务：

```rust,ignore
use tokio::task;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let handle = task::spawn(async {
        sleep(Duration::from_secs(1)).await;
        "任务结果"
    });
    
    println!("任务已生成，正在做其他工作...");
    
    // 等待任务完成
    let result = handle.await.unwrap();
    println!("任务返回: {}", result);
}
```

### Select 和竞争 Future

执行多个 future，并对第一个完成的采取行动：

```rust,ignore
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    tokio::select! {
        _ = sleep(Duration::from_secs(1)) => {
            println!("已达到超时");
        }
        _ = async {
            sleep(Duration::from_millis(500)).await;
            println!("任务已完成");
        } => {}
    }
}
```

### 异步特征 (限制)

目前，在特征中使用异步函数需要变通方法或使用 `async-trait` crate：

```toml
# 添加到 Cargo.toml
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
        "来自 API 的数据".to_string()
    }
}
```

### 常见的异步运行时

**Tokio**: 最流行，功能丰富，适用于网络服务
**async-std**: 类似标准库的 API，更简单
**smol**: 最小化且高效
**embassy**: 用于嵌入式系统

### 最佳实践

- **对 I/O 密集型工作使用异步** (网络, 文件 I/O, 定时器)
- **对 CPU 密集型工作使用线程** (重度计算)
- **不要在异步代码中阻塞** - 避免 `thread::sleep`，使用 `tokio::time::sleep`
- **注意在循环中使用 `.await`** - 考虑使用 `join!` 或 `select!` 实现并发
- **为您的用例选择合适的运行时**
- **显式处理错误** - future 在被等待之前不会执行
- **在异步上下文中使用 `tokio::spawn` 实现真正的并发**

