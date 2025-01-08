### Depth First Search (DFS) Algorithm

Depth First Search (DFS) is an algorithm for traversing or searching tree or graph data structures. The algorithm starts at the root node (selecting some arbitrary node as the root in the case of a graph) and explores as far as possible along each branch before backtracking.

Below is a simple example of how to implement DFS in Rust. We'll use an adjacency list to represent the graph.

```rust
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
    // Create a sample graph as an adjacency list
    let mut graph = HashMap::new();
    graph.insert(0, vec![1, 2]);
    graph.insert(1, vec![2]);
    graph.insert(2, vec![0, 3]);
    graph.insert(3, vec![3]);

    // Vector to keep track of visited nodes
    let mut visited = Vec::new();

    // Perform DFS starting from node 2
    dfs(&graph, 2, &mut visited);

    // Print the visited nodes
    println!("Visited nodes: {:?}", visited);
}
```

1. **Graph Representation**: The graph is represented using a `HashMap` where each key is a node, and the value is a vector of its neighbors.
2. **DFS Function**: The `dfs` function takes a reference to the graph, the current node, and a mutable reference to a vector of visited nodes.
3. **Base Case**: If the current node has already been visited, the function returns.
4. **Visit Node**: The current node is added to the visited vector.
5. **Recursive Call**: The function recursively visits all the neighbors of the current node.
6. **Main Function**: The `main` function initializes the graph, creates a vector to track visited nodes, and calls the `dfs` function starting from node 2. Finally, it prints the visited nodes.

This example demonstrates a basic implementation of the DFS algorithm in Rust. You can modify the graph and the starting node to see how the traversal changes.