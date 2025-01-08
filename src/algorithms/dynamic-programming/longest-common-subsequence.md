### Longest Common Subsequence (LCS)

The Longest Common Subsequence (LCS) problem is a classic dynamic programming problem. It involves finding the longest subsequence common to two sequences. A subsequence is a sequence derived from another sequence by deleting some elements without changing the order of the remaining elements.

### Explanation

Given two sequences, `X` and `Y`, the LCS problem is to find the longest subsequence that is present in both sequences. For example, if `X = "ABCBDAB"` and `Y = "BDCAB"`, the LCS is `"BCAB"`.

### Dynamic Programming Approach

We can solve the LCS problem using dynamic programming by constructing a 2D table `dp` where `dp[i][j]` represents the length of the LCS of the sequences `X[0..i-1]` and `Y[0..j-1]`.

### Steps:
1. Initialize a 2D array `dp` of size `(m+1) x (n+1)` where `m` and `n` are the lengths of `X` and `Y` respectively.
2. Fill the table using the following rules:
    - If `X[i-1] == Y[j-1]`, then `dp[i][j] = dp[i-1][j-1] + 1`
    - Otherwise, `dp[i][j] = max(dp[i-1][j], dp[i][j-1])`
3. The value at `dp[m][n]` will be the length of the LCS.

Here is a simple Rust implementation of the LCS algorithm:

```rust
fn longest_common_subsequence(X: &str, Y: &str) -> String {
     let m = X.len();
     let n = Y.len();
     let X: Vec<char> = X.chars().collect();
     let Y: Vec<char> = Y.chars().collect();
     
     let mut dp = vec![vec![0; n + 1]; m + 1];
     
     for i in 1..=m {
          for j in 1..=n {
                if X[i - 1] == Y[j - 1] {
                     dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                     dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
                }
          }
     }
     
     // Reconstruct the LCS from the dp table
     let mut lcs = String::new();
     let (mut i, mut j) = (m, n);
     while i > 0 && j > 0 {
          if X[i - 1] == Y[j - 1] {
                lcs.push(X[i - 1]);
                i -= 1;
                j -= 1;
          } else if dp[i - 1][j] > dp[i][j - 1] {
                i -= 1;
          } else {
                j -= 1;
          }
     }
     
     lcs.chars().rev().collect()
}

fn main() {
     let X = "ABCBDAB";
     let Y = "BDCAB";
     let lcs = longest_common_subsequence(X, Y);
     println!("The Longest Common Subsequence is: {}", lcs);
}
```

1. **Initialization**: We initialize the `dp` table with zeros.
2. **Filling the Table**: We fill the table based on the rules mentioned above.
3. **Reconstructing the LCS**: We backtrack from `dp[m][n]` to reconstruct the LCS.

This implementation prints the LCS of the given sequences `X` and `Y`.
