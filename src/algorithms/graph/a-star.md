### A* Pathfinding Algorithm

The A* (A-star) algorithm is a popular pathfinding and graph traversal algorithm. It is widely used in various applications, such as in games for NPC movement, in robotics for navigation, and in geographic information systems for route planning. A* combines the strengths of Dijkstra's Algorithm and Greedy Best-First-Search by using a heuristic to guide its search efficiently.

A* algorithm maintains two lists:
- **Open List**: Nodes that need to be evaluated (implemented as a priority queue).
- **Closed List**: Nodes that have already been evaluated.

The algorithm follows these steps:
1. Add the starting node to the open list.
2. Repeat the following steps until the open list is empty:
    - a. Remove the node with the lowest `f` value from the open list (current node).
    - b. If the current node is the goal, reconstruct the path and return it.
    - c. For each neighbor of the current node:
      - i. If the neighbor is in the closed list, skip it.
      - ii. If the neighbor is not in the open list, add it and compute its `f`, `g`, and `h` values.
      - iii. If the neighbor is in the open list, check if the new path is better (lower `g` value). If so, update its `f`, `g`, and `h` values.
    - d. Add the current node to the closed list.

Where:
- `g` is the cost from the start node to the current node.
- `h` is the heuristic estimate of the cost from the current node to the goal (using Manhattan distance in this example).
- `f` is the sum of `g` and `h`, representing the estimated total cost.

```rust
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
     position: (i32, i32),
     g: i32,
     h: i32,
}

impl Node {
     fn f(&self) -> i32 {
          self.g + self.h
     }
}

impl Ord for Node {
     fn cmp(&self, other: &Self) -> Ordering {
          other.f().cmp(&self.f())
     }
}

impl PartialOrd for Node {
     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
          Some(self.cmp(other))
     }
}

fn heuristic(a: (i32, i32), b: (i32, i32)) -> i32 {
     (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn a_star(start: (i32, i32), goal: (i32, i32), obstacles: &[(i32, i32)]) -> Option<Vec<(i32, i32)>> {
     let mut open_list = BinaryHeap::new();
     let mut closed_list = HashMap::new();
     let mut came_from = HashMap::new();

     open_list.push(Node {
          position: start,
          g: 0,
          h: heuristic(start, goal),
     });

     while let Some(current) = open_list.pop() {
          if current.position == goal {
                let mut path = vec![current.position];
                let mut current_position = current.position;
                while let Some(&prev_position) = came_from.get(&current_position) {
                     path.push(prev_position);
                     current_position = prev_position;
                }
                path.reverse();
                return Some(path);
          }

          closed_list.insert(current.position, current.g);

          for &neighbor in &[
                (current.position.0 - 1, current.position.1),
                (current.position.0 + 1, current.position.1),
                (current.position.0, current.position.1 - 1),
                (current.position.0, current.position.1 + 1),
          ] {
                if obstacles.contains(&neighbor) || closed_list.contains_key(&neighbor) {
                     continue;
                }

                let tentative_g = current.g + 1;

                if let Some(&existing_g) = closed_list.get(&neighbor) {
                     if tentative_g >= existing_g {
                          continue;
                     }
                }

                open_list.push(Node {
                     position: neighbor,
                     g: tentative_g,
                     h: heuristic(neighbor, goal),
                });

                came_from.insert(neighbor, current.position);
          }
     }

     None
}

fn main() {
     let start = (0, 0);
     let goal = (4, 4);
     let obstacles = [(1, 1), (2, 2), (3, 3)];

     if let Some(path) = a_star(start, goal, &obstacles) {
          println!("Path found: {:?}", path);
     } else {
          println!("No path found.");
     }
}
```

- The `Node` struct represents a position in the search space with `g` (cost from start) and `h` (heuristic estimate to goal) values. It implements `Ord` to enable the binary heap to work as a min-heap based on `f` values.
- The `heuristic` function calculates the Manhattan distance between two points, which is admissible for grid-based pathfinding.
- The `a_star` function implements the A* algorithm using a binary heap as the open list and a hash map as the closed list.
- The algorithm explores neighbors in four directions (up, down, left, right), avoiding obstacles and already-evaluated nodes.
- When the goal is reached, the path is reconstructed by backtracking through the `came_from` map.
- The `main` function demonstrates finding a path from (0,0) to (4,4) while avoiding obstacles at (1,1), (2,2), and (3,3).
