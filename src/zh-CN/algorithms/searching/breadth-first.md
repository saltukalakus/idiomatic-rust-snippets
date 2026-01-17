### 广度优先搜索（BFS）算法

广度优先搜索（BFS）是一种用于遍历或搜索树或图数据结构的算法。它从树的根（或图的任意节点）开始，并在移动到下一深度级别的节点之前，探索当前深度的所有邻居节点。

```rust, editable
use std::collections::{HashMap, VecDeque};

fn bfs(graph: &HashMap<i32, Vec<i32>>, start: i32) -> Vec<i32> {
    let mut visited = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        if !visited.contains(&node) {
            visited.push(node);
            if let Some(neighbors) = graph.get(&node) {
                for &neighbor in neighbors {
                    queue.push_back(neighbor);
                }
            }
        }
    }

    visited
}

fn main() {
    let mut graph = HashMap::new();
    graph.insert(1, vec![2, 3]);
    graph.insert(2, vec![4, 5]);
    graph.insert(3, vec![6, 7]);
    graph.insert(4, vec![]);
    graph.insert(5, vec![]);
    graph.insert(6, vec![]);
    graph.insert(7, vec![]);

    let start_node = 1;
    let result = bfs(&graph, start_node);
    println!("从节点 {} 开始的 BFS 遍历: {:?}", start_node, result);
}
```

- 图使用 `HashMap` 表示，其中每个键是一个节点，值是其邻居的向量。
- `bfs` 函数接受对图的引用和起始节点，并按 BFS 顺序返回已访问节点的向量。
- `VecDeque` 被用作队列来管理待探索的节点，遵循 BFS 至关重要的先进先出（FIFO）原则。
- 函数迭代队列，检查每个节点是否已被访问。如果没有，它将节点标记为已访问，并将其所有未访问的邻居添加到队列中。
- 该算法确保每个节点只被访问一次，从起始节点开始逐层处理节点。
- `main` 函数初始化一个包含7个节点的示例图，从节点1开始调用 `bfs` 函数，并打印 BFS 遍历顺序。
