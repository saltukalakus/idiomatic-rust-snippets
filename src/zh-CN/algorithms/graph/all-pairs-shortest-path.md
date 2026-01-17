````markdown
### Floyd–Warshall 算法（全源最短路径）

全源最短路径（APSP）算法用于求图中任意两点之间的最短路径。Floyd–Warshall 算法是使用动态规划解决该问题的经典方法。

算法思想：将每个节点作为中间节点逐步考虑，更新距离矩阵以构建最短路径。

1. **初始化**：创建距离矩阵 `dist`，`dist[i][j]` 表示从节点 `i` 到节点 `j` 的最短距离。若存在边则初始化为边权，否则设为无穷大；对角线 `dist[i][i]` 设为 0。

2. **迭代更新**：对每个中间节点 `k`，对任意节点对 `(i,j)`，更新 `dist[i][j] = min(dist[i][j], dist[i][k] + dist[k][j])`。

3. **结果**：考虑完所有中间节点后，`dist` 包含所有点对之间的最短距离。

```rust, editable
const INF: i32 = i32::MAX / 2; // Use a large value to represent infinity

fn floyd_warshall(graph: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = graph.len();
    let mut dist = graph.clone();

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if dist[i][j] > dist[i][k] + dist[k][j] {
                    dist[i][j] = dist[i][k] + dist[k][j];
                }
            }
        }
    }

    dist
}

fn main() {
    let graph = vec![
        vec![0, 3, INF, 5],
        vec![2, 0, INF, 4],
        vec![INF, 1, 0, INF],
        vec![INF, INF, 2, 0],
    ];

    let shortest_paths = floyd_warshall(&graph);

    println!("Shortest distances between every pair of vertices:");
    for row in shortest_paths {
        for &dist in &row {
            if dist == INF {
                print!("INF ");
            } else {
                print!("{} ", dist);
            }
        }
        println!();
    }
}
```

- 图以邻接矩阵表示，`graph[i][j]` 为节点 `i` 到 `j` 的边权；若无边，则设为 `INF`（表示无穷大）。
- `floyd_warshall` 函数通过三重循环对每个中间节点进行更新，逐步考虑通过该中间节点是否能缩短两点之间的距离。
- 算法时间复杂度为 O(n³)，适用于小到中等规模的稠密图。
- `main` 函数构造了一个包含 4 个节点的示例有向加权图，并打印所有点对之间的最短距离。
````