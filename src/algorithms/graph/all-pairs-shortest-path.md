### Floyd-Warshall Algorithm

The All Pairs Shortest Path (APSP) algorithm is used to find the shortest paths between all pairs of nodes in a graph. The Floyd-Warshall algorithm is one of the most efficient algorithms to solve this problem using dynamic programming.

The Floyd-Warshall algorithm uses dynamic programming to build up the shortest paths by considering each node as an intermediate point. The algorithm follows these steps:

1. **Initialization**: Create a distance matrix `dist` where `dist[i][j]` represents the shortest distance from node `i` to node `j`. Initialize `dist[i][j]` to the weight of the edge from `i` to `j` if it exists, otherwise set it to infinity. Set `dist[i][i]` to 0 for all nodes `i`.

2. **Iterative Update**: For each node `k`, update the distance matrix by considering `k` as an intermediate node. For each pair of nodes `(i, j)`, update `dist[i][j]` to be the minimum of `dist[i][j]` and `dist[i][k] + dist[k][j]`.

3. **Result**: After considering all nodes as intermediate points, the distance matrix `dist` will contain the shortest distances between all pairs of nodes.

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

- The graph is represented as an adjacency matrix where `graph[i][j]` is the weight of the edge from node `i` to node `j`. If there is no edge, it is set to `INF` (a large value representing infinity).
- The `floyd_warshall` function implements the algorithm using three nested loops to consider each node as an intermediate point.
- For each pair of nodes `(i, j)`, the algorithm checks if going through node `k` provides a shorter path than the current known path.
- The algorithm has a time complexity of O(n³), making it efficient for small to medium-sized dense graphs.
- The `main` function creates a sample directed weighted graph with 4 nodes and runs the algorithm, printing the shortest distances between all pairs of nodes.