### Minimum Spanning Tree (MST)

A Minimum Spanning Tree (MST) of a graph is a subset of the edges that connects all the vertices together, without any cycles and with the minimum possible total edge weight. 

Kruskal's algorithm is one of the most efficient algorithms to find the MST. The algorithm follows these steps:

1. Sort all the edges in non-decreasing order of their weight.
2. Pick the smallest edge. Check if it forms a cycle with the spanning tree formed so far. If a cycle is not formed, include this edge. Else, discard it.
3. Repeat step 2 until there are (V-1) edges in the spanning tree.

```rust
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

- The graph is represented as a collection of edges, where each edge has a source node, destination node, and weight.
- The `Graph` struct stores the number of vertices and a vector of all edges.
- The `find` function implements the "find" operation of the Union-Find data structure, which determines the root parent of a node.
- The `union` function merges two subsets using union by rank optimization.
- The `kruskal_mst` function sorts all edges by weight and iteratively adds the smallest edge that doesn't create a cycle.
- The algorithm uses Union-Find to efficiently detect cycles: if two nodes have the same root parent, adding an edge between them would create a cycle.
- The `main` function creates a sample weighted graph with 4 vertices and 5 edges, runs Kruskal's algorithm, and prints the edges in the MST.