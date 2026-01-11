### Breadth-First Search (BFS) Algorithm

Breadth-First Search (BFS) is an algorithm for traversing or searching tree or graph data structures. It starts at the tree root (or an arbitrary node of a graph) and explores all neighbor nodes at the present depth before moving on to nodes at the next depth level.

```rust
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
    println!("BFS traversal starting from node {}: {:?}", start_node, result);
}
```

- The graph is represented using a `HashMap` where each key is a node, and the value is a vector of its neighbors.
- The `bfs` function takes a reference to the graph and a starting node, returning a vector of visited nodes in BFS order.
- A `VecDeque` is used as a queue to manage the nodes to be explored, following the First-In-First-Out (FIFO) principle essential for BFS.
- The function iterates over the queue, checking if each node has been visited. If not, it marks the node as visited and adds all its unvisited neighbors to the queue.
- The algorithm ensures each node is visited exactly once, processing nodes level by level from the starting node.
- The `main` function initializes a sample graph with 7 nodes, calls the `bfs` function starting from node 1, and prints the BFS traversal order.