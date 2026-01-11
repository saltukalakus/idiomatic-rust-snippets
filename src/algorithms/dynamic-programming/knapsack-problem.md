### Knapsack Problem

The Knapsack Problem is a classic dynamic programming problem. Given a set of items, each with a weight and a value, the goal is to select items to maximize the total value while keeping the total weight within a given limit.

The solution uses a 2D array `dp` where `dp[i][w]` represents the maximum value achievable with the first `i` items and a knapsack capacity of `w`. For each item, we decide whether to include it or exclude it based on which choice yields a higher value.

```rust
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

- The `knapsack` function takes slices of weights and values, plus the knapsack capacity.
- A 2D vector `dp` with dimensions `(n+1) x (capacity+1)` is initialized with zeros.
- For each item `i` and capacity `w`, the algorithm decides: if the item's weight fits (`weights[i-1] <= w`), choose the maximum between including the item (`dp[i-1][w-weights[i-1]] + values[i-1]`) or excluding it (`dp[i-1][w]`).
- If the item doesn't fit, carry forward the previous best value (`dp[i-1][w]`).
- The value at `dp[n][capacity]` contains the maximum achievable value.
- The `main` function demonstrates the algorithm with 4 items and a capacity of 7, printing the maximum value of 9.
