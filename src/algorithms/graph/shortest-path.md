### Shortest Path Graph (Dijkstra's) Algorithm

Dijkstra's algorithm is used to find the shortest path between nodes in a graph. It works by maintaining a set of unvisited nodes and iteratively selecting the node with the smallest tentative distance, updating the distances to its neighbors, and marking it as visited. 

```rust
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

- The graph is represented as an adjacency list where each node has a list of edges. Each edge has a target node and a cost.
- The `State` struct keeps track of the current position and the cost to reach that position. It implements `Ord` and `PartialOrd` to enable the binary heap to function as a min-heap.
- A `BinaryHeap` is used as a priority queue to always expand the least costly node first.
- The `dijkstra` function initializes the distance to the start node as 0 and all other distances as `usize::MAX` (infinity).
- The algorithm iteratively explores the graph, updating the shortest path to each node using edge relaxation.
- The `main` function creates a sample graph, runs Dijkstra's algorithm from node 0, and prints the shortest distances to all other nodes.