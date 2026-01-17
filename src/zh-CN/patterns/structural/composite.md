### 组合（Composite）模式

组合模式允许将对象组合成树形结构以表示“部分-整体”的层次结构。Rust 中常用枚举或递归结构来表示组合。

```rust, editable
enum Node { Leaf(i32), Branch(Box<Node>, Box<Node>) }

fn sum(n: &Node) -> i32 { match n { Node::Leaf(v) => *v, Node::Branch(l,r) => sum(l)+sum(r) } }
```
