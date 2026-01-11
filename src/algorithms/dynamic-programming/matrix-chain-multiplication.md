### Matrix Chain Multiplication

Matrix Chain Multiplication is a classic dynamic programming problem that determines the most efficient way to multiply a sequence of matrices. The goal is not to perform the actual multiplications, but to find the order that minimizes the total number of scalar multiplications. For example, with matrices of dimensions 10x30, 30x5, and 5x60, different parenthesizations result in different numbers of operations.

The algorithm builds a table `m` where `m[i][j]` represents the minimum cost of multiplying matrices from index `i` to `j`. It considers all possible split points to find the optimal parenthesization.

```rust
fn matrix_chain_order(p: &[usize]) -> Vec<Vec<usize>> {
    let n = p.len() - 1;
    let mut m = vec![vec![0; n]; n];

    for l in 2..=n {
        for i in 0..n - l + 1 {
            let j = i + l - 1;
            m[i][j] = usize::MAX;
            for k in i..j {
                let q = m[i][k] + m[k + 1][j] + p[i] * p[k + 1] * p[j + 1];
                if q < m[i][j] {
                    m[i][j] = q;
                }
            }
        }
    }

    m
}

fn main() {
    let p = vec![10, 30, 5, 60];
    let m = matrix_chain_order(&p);
    println!("Minimum number of multiplications is {}", m[0][p.len() - 2]);
}
```

- The function takes an array `p` where the `i`-th matrix has dimensions `p[i-1] x p[i]`. For example, `[10, 30, 5, 60]` represents three matrices: 10×30, 30×5, and 5×60.
- A 2D vector `m` is initialized with zeros, where `m[i][j]` will store the minimum number of scalar multiplications needed to compute matrices from `i` to `j`.
- The outer loop `l` iterates over chain lengths from 2 to `n` (number of matrices).
- For each chain length, the algorithm tries all possible split points `k` between `i` and `j`.
- For each split point, it calculates the cost: `m[i][k] + m[k+1][j] + p[i] * p[k+1] * p[j+1]`, representing the cost of two sub-chains plus the cost of multiplying the results.
- The minimum cost among all split points is stored in `m[i][j]`.
- The `main` function demonstrates finding the minimum multiplications for three matrices (10×30, 30×5, 5×60), which is 4500.
