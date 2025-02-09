### Matrix Chain Multiplication (WIP)

Matrix Chain Multiplication is a classic example of dynamic programming. The goal is to find the most efficient way to multiply a given sequence of matrices. The problem is not to perform the multiplications but to decide the order in which to perform the multiplications.

### Problem Explanation

Given a sequence of matrices, the task is to find the most efficient way to multiply these matrices together. The efficiency is measured in terms of the number of scalar multiplications needed.

For example, if you have matrices A, B, and C of dimensions 10x30, 30x5, and 5x60 respectively, you need to determine the order of multiplication that minimizes the total number of scalar multiplications.

### Dynamic Programming Solution

The dynamic programming approach involves breaking the problem into smaller subproblems and solving each subproblem only once, storing the results for future use.

### Steps:

1. Define the cost of multiplying matrices from index `i` to `j` as `m[i][j]`.
2. Initialize the cost of multiplying one matrix as zero, i.e., `m[i][i] = 0`.
3. Use a nested loop to calculate the minimum cost for chains of increasing length.
4. Store the results in a table and use them to build up the solution for the entire chain.

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

- `p` is an array where the `i`-th matrix has dimensions `p[i-1] x p[i]`.
- `m[i][j]` stores the minimum number of scalar multiplications needed to compute the matrix `A[i]A[i+1]...A[j]`.
- The nested loops calculate the minimum cost for multiplying matrices from `i` to `j` by trying all possible positions to split the product.

This implementation efficiently computes the minimum number of multiplications needed to multiply the given sequence of matrices.
