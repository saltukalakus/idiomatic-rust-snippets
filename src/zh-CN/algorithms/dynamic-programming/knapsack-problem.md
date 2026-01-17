````markdown
### 背包问题（Knapsack Problem）

背包问题是经典的动态规划问题。给定一组物品，每个物品有重量和价值，目标是在限定的容量下选择物品使总价值最大。

该解法使用 2D 数组 `dp`，其中 `dp[i][w]` 表示在前 `i` 个物品和容量为 `w` 的条件下可获得的最大价值。对于每个物品，决定是否将其放入背包，取两种选择中的较大值。

```rust, editable
fn knapsack(weights: &[usize], values: &[usize], capacity: usize) -> usize {
    let n = weights.len();
    let mut dp = vec![vec![0; capacity + 1]; n + 1];

    for i in 1..=n {
        for w in 1..=capacity {
            if weights[i - 1] <= w {
                dp[i][w] = dp[i - 1][w].max(dp[i - 1][w - weights[i - 1]] + values[i - 1]);
            } else {
                dp[i][w] = dp[i - 1][w];
            }
        }
    }

    dp[n][capacity]
}

fn main() {
    let weights = vec![1, 3, 4, 5];
    let values = vec![1, 4, 5, 7];
    let capacity = 7;
    let max_value = knapsack(&weights, &values, capacity);
    println!("The maximum value that can be obtained is: {}", max_value);
}
```

- `knapsack` 函数接受权重切片、价值切片和背包容量。
- 2D 向量 `dp` 的维度为 `(n+1) x (capacity+1)`，初始化为 0。
- 对于每个物品 `i` 和容量 `w`，算法判断：若物品重量满足 (`weights[i-1] <= w`)，则在包含该物品（`dp[i-1][w-weights[i-1]] + values[i-1]`）与不包含该物品（`dp[i-1][w]`）之间取较大值。
- 若物品放不下，则继承之前的最优值（`dp[i-1][w]`）。
- `dp[n][capacity]` 存储最大可达总价值。
- `main` 函数使用 4 个物品和容量 7 演示算法，输出最大值 9。

````