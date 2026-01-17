### 深度优先搜索（Depth-First Search, DFS）

DFS 递归或使用栈深入探索图或树的分支，适合需要遍历所有路径或进行拓扑排序的场景。需注意递归深度与已访问集合以避免无限循环。

```rust, editable
fn dfs(u: usize, adj: &Vec<Vec<usize>>, visited: &mut Vec<bool>, order: &mut Vec<usize>) {
    visited[u] = true;
    order.push(u);
    for &v in &adj[u] {
        if !visited[v] { dfs(v, adj, visited, order); }
    }
}

fn depth_first(adj: &Vec<Vec<usize>>, start: usize) -> Vec<usize> {
    let mut visited = vec![false; adj.len()];
    let mut order = Vec::new();
    dfs(start, adj, &mut visited, &mut order);
    order
}
```
