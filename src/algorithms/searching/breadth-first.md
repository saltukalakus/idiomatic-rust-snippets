### Breadth-First Search (BFS) Algorithm

Breadth-First Search (BFS) is an algorithm for traversing or searching tree or graph data structures. It starts at the tree root (or an arbitrary node of a graph) and explores all neighbor nodes at the present depth before moving on to nodes at the next depth level.

```rust, editable
use std::collections::{HashMap, HashSet, VecDeque};

fn bfs<F>(graph: &HashMap<i32, Vec<i32>>, start: i32, mut visit: F)
where
    F: FnMut(i32),
{
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        if visited.insert(node) {
            visit(node);
            if let Some(neighbors) = graph.get(&node) {
                for &neighbor in neighbors {
                    queue.push_back(neighbor);
                }
            }
        }
    }
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
    print!("BFS traversal starting from node {start_node}:");
    bfs(&graph, start_node, |node| print!(" {node}"));
    println!();
}
```

- The graph is represented using a `HashMap` where each key is a node, and the value is a vector of its neighbors.
- The `bfs` function accepts a generic callback `visit: F` (any `FnMut(i32)`) that is called once per node in traversal order, avoiding the need to allocate and return a separate collection.
- A `VecDeque` is used as a queue to manage the nodes to be explored, following the First-In-First-Out (FIFO) principle essential for BFS.
- A `HashSet` tracks visited nodes for O(1) membership checks. `HashSet::insert` returns `true` if the node was newly inserted, so a single call both checks and marks the node as visited.
- The algorithm ensures each node is visited exactly once, processing nodes level by level from the starting node.
- The `main` function initializes a sample graph with 7 nodes, then passes a closure to `bfs` that prints each node as it is visited.