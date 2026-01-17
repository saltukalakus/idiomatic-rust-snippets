### 最短路径图（Dijkstra 算法）

Dijkstra 算法用于查找图中节点之间的最短路径。它通过维护一组未访问的节点并迭代选择具有最小暂定距离的节点来工作，随后更新其邻居的距离并将其标记为已访问。

```rust, editable
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

// 优先队列依赖于 `Ord`。
// 显式实现该 trait，以便队列成为最小堆而不是最大堆。
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // 注意我们反转了成本的排序。
        // 如果成本相同，我们比较位置 - 这是为了使 `PartialEq` 和 `Ord` 的实现保持一致所必需的。
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` 也需要实现。
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Edge {
    node: usize,
    cost: usize,
}

fn dijkstra(adj_list: &Vec<Vec<Edge>>, start: usize) -> Vec<usize> {
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();
    let mut heap = BinaryHeap::new();

    // 我们在 `start` 位置，成本为零
    dist[start] = 0;
    heap.push(State { cost: 0, position: start });

    // 首先检查成本较低的节点（最小堆）
    while let Some(State { cost, position }) = heap.pop() {
        // 很重要，因为我们可能已经找到了更好的路径
        if cost > dist[position] {
            continue;
        }

        // 对于我们可以到达的每个节点，看看是否可以通过此节点找到成本更低的路径
        for edge in &adj_list[position] {
            let next = State { cost: cost + edge.cost, position: edge.node };

            // 如果是，则将其添加到边界并继续
            if next.cost < dist[next.position] {
                // 松弛操作，我们现在找到了更好的路径
                dist[next.position] = next.cost;
                heap.push(next);
            }
        }
    }

    dist
}

fn main() {
    // 创建一个表示为邻接表的图
    let graph = vec![
        vec![Edge { node: 1, cost: 2 }, Edge { node: 2, cost: 4 }],
        vec![Edge { node: 2, cost: 1 }, Edge { node: 3, cost: 7 }],
        vec![Edge { node: 3, cost: 3 }],
        vec![],
    ];

    // 计算从节点 0 开始的最短路径
    let start_node = 0;
    let distances = dijkstra(&graph, start_node);

    // 打印到每个节点的最短路径
    for (i, d) in distances.iter().enumerate() {
        println!("从节点 {} 到节点 {} 的最短路径是 {}", start_node, i, d);
    }
}
```

- 图使用邻接表表示，每个节点有一组边。每条边包含目标节点和代价（权重）。
- `State` 结构记录当前位置及到达该位置的代价。它实现了 `Ord` 和 `PartialOrd`，以便二叉堆可以作为最小堆使用。
- 使用 `BinaryHeap` 作为优先队列，总是先展开代价最小的节点。
- `dijkstra` 函数将起始节点的距离初始化为 0，其他节点距离设为 `usize::MAX`（表示无穷大）。
- 算法通过边的松弛（relaxation）迭代更新到各节点的最短路径。
- `main` 函数创建了一个示例图，从节点 0 运行 Dijkstra 算法并打印到各节点的最短距离。

