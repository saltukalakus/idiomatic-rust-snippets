### 深度优先搜索（DFS）算法

深度优先搜索（DFS）是一种用于遍历或搜索树或图数据结构的算法。该算法从根节点（在图的情况下选择某个任意节点作为根）开始，并沿着每个分支尽可能远地探索，直到回溯。

```rust, editable
use std::collections::HashMap;

fn dfs(graph: &HashMap<i32, Vec<i32>>, start: i32, visited: &mut Vec<i32>) {
    if visited.contains(&start) {
        return;
    }
    visited.push(start);
    if let Some(neighbors) = graph.get(&start) {
        for &neighbor in neighbors {
            dfs(graph, neighbor, visited);
        }
    }
}

fn main() {
    // 创建一个示例图作为邻接表
    let mut graph = HashMap::new();
    graph.insert(0, vec![1, 2]);
    graph.insert(1, vec![2]);
    graph.insert(2, vec![0, 3]);
    graph.insert(3, vec![3]);

    // 用于跟踪已访问节点的向量
    let mut visited = Vec::new();

    // 从节点 2 开始执行 DFS
    dfs(&graph, 2, &mut visited);

    // 打印已访问的节点
    println!("已访问节点: {:?}", visited);
}
```

- 图使用 `HashMap` 表示，其中每个键是一个节点，值是其邻居的向量。
- `dfs` 函数接受对图的引用、当前节点以及对已访问节点向量的可变引用。
- 如果当前节点已被访问，函数将返回。
- 当前节点被添加到已访问向量中。
- 函数递归地访问当前节点的所有邻居。
- `main` 函数初始化图，创建一个向量来跟踪已访问的节点，并从节点 2 开始调用 `dfs` 函数。最后，它打印已访问的节点。
