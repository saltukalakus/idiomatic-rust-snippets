### 最小生成树（MST）

最小生成树（MST）是图的一个边子集，它连接了图中所有顶点且不形成环，并使总边权最小。

Kruskal 算法是求解最小生成树的高效方法之一，基本步骤如下：

1. 将所有边按权重非降序排序。
2. 依次选择最小的边，检查是否与当前生成树形成环；若不形成环，则选取该边，否则丢弃。
3. 重复步骤 2，直到生成树包含 (V-1) 条边。

```rust, editable
#[derive(Debug, Clone)]
struct Edge {
    src: usize,
    dest: usize,
    weight: i32,
}

#[derive(Debug)]
struct Graph {
    vertices: usize,
    edges: Vec<Edge>,
}

impl Graph {
    fn new(vertices: usize) -> Self {
        Graph {
            vertices,
            edges: Vec::new(),
        }
    }

    fn add_edge(&mut self, src: usize, dest: usize, weight: i32) {
        self.edges.push(Edge { src, dest, weight });
    }

    fn find(&self, parent: &mut Vec<usize>, i: usize) -> usize {
        if parent[i] == i {
            i
        } else {
            self.find(parent, parent[i])
        }
    }

    fn union(&self, parent: &mut Vec<usize>, rank: &mut Vec<usize>, x: usize, y: usize) {
        let xroot = self.find(parent, x);
        let yroot = self.find(parent, y);

        if rank[xroot] < rank[yroot] {
            parent[xroot] = yroot;
        } else if rank[xroot] > rank[yroot] {
            parent[yroot] = xroot;
        } else {
            parent[yroot] = xroot;
            rank[xroot] += 1;
        }
    }

    fn kruskal_mst(&self) -> Vec<Edge> {
        let mut result: Vec<Edge> = Vec::new();
        let mut i = 0;
        let mut e = 0;

        let mut edges = self.edges.clone();
        edges.sort_by(|a, b| a.weight.cmp(&b.weight));

        let mut parent: Vec<usize> = Vec::with_capacity(self.vertices);
        let mut rank: Vec<usize> = Vec::with_capacity(self.vertices);

        for node in 0..self.vertices {
            parent.push(node);
            rank.push(0);
        }

        while e < self.vertices - 1 {
            let next_edge = edges[i].clone();
            i += 1;

            let x = self.find(&mut parent, next_edge.src);
            let y = self.find(&mut parent, next_edge.dest);

            if x != y {
                result.push(next_edge);
                e += 1;
                self.union(&mut parent, &mut rank, x, y);
            }
        }

        result
    }
}

fn main() {
    let mut graph = Graph::new(4);
    graph.add_edge(0, 1, 10);
    graph.add_edge(0, 2, 6);
    graph.add_edge(0, 3, 5);
    graph.add_edge(1, 3, 15);
    graph.add_edge(2, 3, 4);

    let mst = graph.kruskal_mst();
    println!("Edges in the constructed MST:");
    for edge in mst {
        println!("{} -- {} == {}", edge.src, edge.dest, edge.weight);
    }
}
```

- 图以边集合表示，每条边包含源节点、目标节点和权重。
- `Graph` 结构体存储顶点数量与所有边的向量。
- `find` 函数实现了并查集（Union-Find）结构的查找操作，用于找到节点的根父节点。
- `union` 函数使用按秩合并（union by rank）优化合并两个子集。
- `kruskal_mst` 函数将所有边按权重排序，并迭代加入不会产生环的最小边。
- 算法使用并查集高效检测是否会形成环：若两个节点具有相同的根父节点，则加入该边会形成环。
- `main` 函数构造了一个包含 4 个顶点、5 条边的示例加权图，运行 Kruskal 算法并打印生成树中的边。
