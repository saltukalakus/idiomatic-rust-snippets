### 最长公共子序列（LCS）

最长公共子序列（LCS）问题是经典的动态规划问题，寻找两个序列的最长公共子序列。子序列通过删除某些元素而不改变剩余元素顺序得到。例如，对于 `X = "ABCBDAB"` 和 `Y = "BDCAB"`，一个 LCS 是 `"BDAB"`，长度为 4。

解法构建了 2D 表 `dp`，其中 `dp[i][j]` 表示序列 `X[0..i-1]` 与 `Y[0..j-1]` 的 LCS 长度。表格按照规则填充：若字符匹配，则对角线值加一；否则取左侧或上方的较大值。

```rust, editable
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

- 函数接收两个字符串切片并将其转换为字符向量以便索引访问。
- 2D 向量 `dp` 的大小为 `(m+1) x (n+1)`，其中 `m` 和 `n` 分别为输入字符串的长度，初始值为 0。
- 嵌套循环填充表格：若位置 `i-1` 与 `j-1` 的字符匹配，则 `dp[i][j] = dp[i-1][j-1] + 1`；否则 `dp[i][j] = max(dp[i-1][j], dp[i][j-1])`。
- 构建完表格后，从 `dp[m][n]` 回溯以重构实际的 LCS 字符串。
- 回溯过程中若字符匹配则将其加入结果并同时减小两个索引；否则向值较大的方向移动。
- 由于回溯产生的结果是倒序的，需要在返回前进行反转。
- `main` 函数演示对 "ABCBDAB" 与 "BDCAB" 的 LCS，结果为 "BDAB"（注意：可能存在多个等长的有效 LCS）。

