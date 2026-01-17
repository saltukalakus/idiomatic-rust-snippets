### 命令（Command）模式

命令模式将操作封装为对象，从而实现撤销、重做与队列化执行。在 Rust 中常使用闭包或实现 trait 的类型作为命令。

```rust, editable
trait Command { fn execute(&self); }

struct PrintCommand(String);
impl Command for PrintCommand { fn execute(&self) { println!("{}", self.0); } }

fn main() {
    let cmds: Vec<Box<dyn Command>> = vec![Box::new(PrintCommand("Hello".into()))];
    for c in cmds { c.execute(); }
}
```
