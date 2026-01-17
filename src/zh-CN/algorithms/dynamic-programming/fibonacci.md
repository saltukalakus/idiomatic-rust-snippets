### 斐波那契序列

斐波那契序列是一列数字，其中每个数字是前两个数字之和，通常以 0 和 1 开始。本实现使用动态规划通过保存已计算的值来高效计算斐波那契数，避免重复计算。

```rust, editable
fn fibonacci(n: usize) -> usize {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut fib = vec![0; n + 1];
    fib[1] = 1;

    for i in 2..=n {
        fib[i] = fib[i - 1] + fib[i - 2];
    }

    fib[n]
}

fn main() {
    let n = 10;
    println!("Fibonacci number at position {} is {}", n, fibonacci(n));
}
```

- 函数首先检查 `n` 是否为 0 或 1，分别返回 0 或 1，因为这是斐波那契序列的基例。
- 向量 `fib` 初始化为 `n + 1` 个元素，均为 0，第二个元素设为 1，表示第一个斐波那契数。
- 循环从 2 到 `n` 计算每个斐波那契数，将结果存储在向量中。
- 函数返回存储在向量中的第 `n` 个斐波那契数。
- 该方法确保每个斐波那契数只计算一次，时间复杂度为 O(n)，空间复杂度为 O(n)。
- `main` 函数演示计算第 10 个斐波那契数并打印结果。


