# Fibonacci Sequence with Dynamic Programming (WIP)

The Fibonacci sequence is a series of numbers where each number is the sum of the two preceding ones, usually starting with 0 and 1. Dynamic programming is an optimization technique that can be used to solve problems by breaking them down into simpler subproblems and storing the results of these subproblems to avoid redundant computations.

Here is a simple application in Rust to compute the Fibonacci sequence using dynamic programming:

```rust
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

## Explanation

1. **Base Cases**: The function first checks if `n` is 0 or 1, returning 0 or 1 respectively, as these are the base cases of the Fibonacci sequence.
2. **Initialization**: A vector `fib` is initialized with `n + 1` elements, all set to 0. The second element is set to 1, representing the first Fibonacci number.
3. **Iteration**: A loop runs from 2 to `n`, calculating each Fibonacci number by summing the two preceding numbers and storing the result in the vector.
4. **Result**: The function returns the `n`-th Fibonacci number stored in the vector.

This approach ensures that each Fibonacci number is computed only once, resulting in a time complexity of O(n) and a space complexity of O(n).

