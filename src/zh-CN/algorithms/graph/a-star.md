````markdown
### A* 寻路算法

A*（A-star）算法是一种常用的寻路和图遍历算法，广泛用于游戏中的 NPC 移动、机器人导航以及地理信息系统的路径规划。A* 结合了 Dijkstra 算法和贪心最佳优先搜索的优点，通过启发式函数引导搜索，从而提高效率。

A* 算法维护两个集合：
- **Open 列表**：需要评估的节点（通常用优先队列实现）。
- **Closed 列表**：已被评估的节点。

算法步骤：
1. 将起点加入 Open 列表。
2. 重复下列步骤直到 Open 列表为空：
    - a. 从 Open 列表中取出 `f` 值最小的节点（当前节点）。
    - b. 若当前节点为目标节点，则回溯路径并返回。
    - c. 对当前节点的每个邻居：
      - i. 若邻居在 Closed 列表中，跳过。
      - ii. 若邻居不在 Open 列表，将其加入并计算其 `f`、`g`、`h` 值。
      - iii. 若邻居已在 Open 列表，检查新路径是否更优（`g` 值更小），若更优则更新其 `f`、`g`、`h` 值。
    - d. 将当前节点加入 Closed 列表。

其中：
- `g` 表示从起点到当前节点的实际代价。
- `h` 表示从当前节点到目标节点的启发式估计代价（本示例使用曼哈顿距离）。
- `f = g + h` 表示估计的总代价。

```rust, editable
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

- `Node` 结构体表示搜索空间中的位置，并携带 `g`（起点到当前的代价）和 `h`（启发式估计到目标的代价）值。它实现了 `Ord`，使二叉堆能够基于 `f` 值作为最小堆使用。
- `heuristic` 函数计算两点间的曼哈顿距离，适用于网格寻路且具有可采纳性（admissible）。
- `a_star` 函数使用二叉堆作为 Open 列表、哈希表作为 Closed 列表实现了 A* 算法。
- 算法在四个方向（上、下、左、右）探索邻居，避开障碍物和已评估节点。
- 到达目标时，通过 `came_from` 映射回溯构造路径。
- `main` 函数演示在避开 (1,1)、(2,2)、(3,3) 障碍的情况下，从 (0,0) 寻找到 (4,4) 的路径。

````