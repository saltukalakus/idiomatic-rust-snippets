### Knapsack Problem in Rust (WIP)

The Knapsack Problem is a classic dynamic programming problem. Given a set of items, each with a weight and a value, determine the number of each item to include in a collection so that the total weight is less than or equal to a given limit and the total value is as large as possible.

### Problem Statement

You are given weights and values of `n` items, put these items in a knapsack of capacity `W` to get the maximum total value in the knapsack. 

### Dynamic Programming Approach

We can solve this problem using dynamic programming by creating a 2D array `dp` where `dp[i][w]` represents the maximum value that can be obtained with the first `i` items and a knapsack capacity of `w`.

### Steps

1. Initialize a 2D array `dp` with dimensions `(n+1) x (W+1)` where `n` is the number of items and `W` is the maximum capacity of the knapsack.
2. Iterate through each item and each capacity, updating the `dp` array based on whether the current item is included or excluded.
3. The value at `dp[n][W]` will be the maximum value that can be obtained with the given items and knapsack capacity.

Here is a simple Rust implementation of the Knapsack Problem using dynamic programming:

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

- `weights` and `values` are arrays representing the weights and values of the items.
- `capacity` is the maximum weight the knapsack can hold.
- The `knapsack` function initializes the `dp` array and iterates through each item and capacity to fill the `dp` array.
- The `main` function demonstrates how to use the `knapsack` function with a sample input.

This implementation ensures that we find the maximum value that can be obtained without exceeding the knapsack's capacity.
