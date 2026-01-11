### Longest Common Subsequence (LCS)

The Longest Common Subsequence (LCS) problem is a classic dynamic programming problem that finds the longest subsequence common to two sequences. A subsequence is derived by deleting some elements without changing the order of the remaining elements. For example, for `X = "ABCBDAB"` and `Y = "BDCAB"`, one LCS is `"BDAB"` with length 4.

The solution constructs a 2D table `dp` where `dp[i][j]` represents the length of the LCS of the sequences `X[0..i-1]` and `Y[0..j-1]`. The table is filled using the rule: if characters match, increment the diagonal value; otherwise, take the maximum from the left or top cell.

```rust
fn longest_common_subsequence(x: &str, y: &str) -> String {
     let m = x.len();
     let n = y.len();
     let x_chars: Vec<char> = x.chars().collect();
     let y_chars: Vec<char> = y.chars().collect();
     
     let mut dp = vec![vec![0; n + 1]; m + 1];
     
     for i in 1..=m {
          for j in 1..=n {
                if x_chars[i - 1] == y_chars[j - 1] {
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
          if x_chars[i - 1] == y_chars[j - 1] {
                lcs.push(x_chars[i - 1]);
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
     let x = "ABCBDAB";
     let y = "BDCAB";
     let lcs = longest_common_subsequence(x, y);
     println!("The Longest Common Subsequence is: {}", lcs);
}
```

- The function takes two string slices and converts them to character vectors for indexed access.
- A 2D vector `dp` of size `(m+1) x (n+1)` is initialized with zeros, where `m` and `n` are the lengths of the input strings.
- The nested loops fill the table: if characters at positions `i-1` and `j-1` match, `dp[i][j] = dp[i-1][j-1] + 1`; otherwise, `dp[i][j] = max(dp[i-1][j], dp[i][j-1])`.
- After building the table, the algorithm backtracks from `dp[m][n]` to reconstruct the actual LCS string.
- During backtracking, if characters match, they're added to the result and both indices are decremented; otherwise, move in the direction of the larger value.
- The reconstructed LCS is reversed before returning since backtracking builds it backwards.
- The `main` function demonstrates finding the LCS of "ABCBDAB" and "BDCAB", which produces "BDAB" (note: multiple valid LCS of the same length may exist).
