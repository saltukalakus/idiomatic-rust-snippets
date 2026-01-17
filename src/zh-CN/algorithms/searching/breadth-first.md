### 广度优先搜索（Breadth-First Search, BFS）

BFS 以层级顺序遍历图或树，常用于寻找最短路径（无权图）。使用队列来跟踪待访问节点，并记录已访问集合以防重复。

```rust, editable
use std::collections::{VecDeque, HashSet};

fn bfs(adj: &Vec<Vec<usize>>, start: usize) -> Vec<usize> {
    let mut visited = vec![false; adj.len()];
    let mut order = Vec::new();
    let mut q = VecDeque::new();

    visited[start] = true;
    q.push_back(start);

    while let Some(u) = q.pop_front() {
        order.push(u);
        for &v in &adj[u] {
            if !visited[v] {
                visited[v] = true;
                q.push_back(v);
            }
        }
    }

    order
}
```
