### 矩阵链乘（Matrix Chain Multiplication）

矩阵链乘是经典的动态规划问题，用于确定一系列矩阵相乘的最优计算顺序。目标不是实际执行乘法，而是找出能最小化标量乘法次数的括号化方式。例如，对于维度为 10x30、30x5、5x60 的矩阵，不同的括号化会导致不同的运算量。

算法构建表 `m`，其中 `m[i][j]` 表示将第 `i` 到 `j` 个矩阵相乘的最小代价。通过尝试所有可能的分割点来找到最优的括号化方式。

```rust, editable
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

- 函数接收数组 `p`，其中第 i 个矩阵的维度为 `p[i-1] x p[i]`。例如 `[10, 30, 5, 60]` 表示三个矩阵：10×30、30×5、5×60。
- 2D 向量 `m` 初始化为 0，其中 `m[i][j]` 存储计算第 i 到 j 个矩阵所需的最小标量乘法次数。
- 外层循环 `l` 表示链长，从 2 到 `n`（矩阵个数）。
- 对于每个链长，算法尝试所有可能的分割点 `k`。
- 对于每个分割点，计算代价：`m[i][k] + m[k+1][j] + p[i] * p[k+1] * p[j+1]`，即两个子链的代价加上合并结果的乘法代价。
- 在所有分割点中取最小值并存入 `m[i][j]`。
- `main` 函数演示了三个矩阵（10×30、30×5、5×60）的最小乘法次数，为 4500。

