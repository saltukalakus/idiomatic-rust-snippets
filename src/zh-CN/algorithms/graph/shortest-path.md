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

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
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

    // We're at `start`, with a zero cost
    dist[start] = 0;
    heap.push(State { cost: 0, position: start });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Important as we may have already found a better way
        if cost > dist[position] {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in &adj_list[position] {
            let next = State { cost: cost + edge.cost, position: edge.node };

            // If so, add it to the frontier and continue
            if next.cost < dist[next.position] {
                // Relaxation, we have now found a better way
                dist[next.position] = next.cost;
                heap.push(next);
            }
        }
    }

    dist
}

fn main() {
    // Create a graph represented as an adjacency list
    let graph = vec![
        vec![Edge { node: 1, cost: 2 }, Edge { node: 2, cost: 4 }],
        vec![Edge { node: 2, cost: 1 }, Edge { node: 3, cost: 7 }],
        vec![Edge { node: 3, cost: 3 }],
        vec![],
    ];

    // Calculate the shortest path from node 0
    let start_node = 0;
    let distances = dijkstra(&graph, start_node);

    // Print the shortest path to each node
    for (i, d) in distances.iter().enumerate() {
        println!("The shortest path from node {} to node {} is {}", start_node, i, d);
    }
}
```

- 图使用邻接表表示，每个节点有一组边。每条边包含目标节点和代价（权重）。
- `State` 结构记录当前位置及到达该位置的代价。它实现了 `Ord` 和 `PartialOrd`，以便二叉堆可以作为最小堆使用。
- 使用 `BinaryHeap` 作为优先队列，总是先展开代价最小的节点。
- `dijkstra` 函数将起始节点的距离初始化为 0，其他节点距离设为 `usize::MAX`（表示无穷大）。
- 算法通过边的松弛（relaxation）迭代更新到各节点的最短路径。
- `main` 函数创建了一个示例图，从节点 0 运行 Dijkstra 算法并打印到各节点的最短距离。

