### 责任链（Chain of Responsibility）

责任链模式将请求沿一条处理链传递，直到某个处理器处理它。Rust 中可用闭包、函数指针或 trait 对象来实现动态链路。

```rust, editable
trait Handler { fn handle(&self, req: &str) -> bool; }

struct Logger;
impl Handler for Logger { fn handle(&self, req: &str) -> bool { println!("日志：{}", req); false } }

struct Auth;
impl Handler for Auth { fn handle(&self, req: &str) -> bool { req == "allowed" } }

fn main() {
    let handlers: Vec<Box<dyn Handler>> = vec![Box::new(Logger), Box::new(Auth)];
    let req = "allowed";
    for h in &handlers { if h.handle(req) { println!("已处理"); break; } }
}
```
