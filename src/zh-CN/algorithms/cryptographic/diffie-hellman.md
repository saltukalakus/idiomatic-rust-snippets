### Diffie-Hellman 密钥交换

Diffie-Hellman 密钥交换算法允许两个通信方在不安全信道上安全地协商共享的对称密钥。其安全性依赖于离散对数问题的难度。

**密钥交换步骤：**
- 双方约定一个大素数 `p` 和基数 `g`（模 p 的原根）。
- A 方生成私钥并计算公钥：`public_A = g^private_A mod p`。
- B 方生成私钥并计算公钥：`public_B = g^private_B mod p`。
- 双方通过不安全信道交换公钥（窃听者可以看到公钥，但无法推导出私钥）。
- A 方计算共享密钥：`secret_A = public_B^private_A mod p`。
- B 方计算共享密钥：`secret_B = public_A^private_B mod p`。
- 由于数学性质 `(g^a)^b = (g^b)^a = g^(ab) mod p`，两个共享密钥是相等的。
- 断言确认双方计算出相同的共享密钥。
- 共享密钥随后可用作对称密钥，用于进一步的加密通信。
- 此示例为清晰起见使用了较小的数字；生产实现应使用像 `ring` 或 `openssl` 这样的加密库，并配合适当大小的参数和安全的随机数生成。

```rust, editable
fn main() {
    // Step 1: Agree on a prime number p and base g
    let p: u64 = 23; // A small prime number for simplicity
    let g: u64 = 5;  // A primitive root modulo p

    // Step 2: Each party generates a private key (using fixed values for demonstration)
    let private_key_a = 6;
    let private_key_b = 15;

    // Step 3: Compute public keys
    let public_key_a = mod_exp(g, private_key_a, p);
    let public_key_b = mod_exp(g, private_key_b, p);

    // Step 4: Exchange public keys (simulated here)
    println!("Public Key A: {}", public_key_a);
    println!("Public Key B: {}", public_key_b);

    // Step 5: Compute shared secret
    let shared_secret_a = mod_exp(public_key_b, private_key_a, p);
    let shared_secret_b = mod_exp(public_key_a, private_key_b, p);

    // Both shared secrets should be the same
    println!("Shared Secret A: {}", shared_secret_a);
    println!("Shared Secret B: {}", shared_secret_b);
    
    assert_eq!(shared_secret_a, shared_secret_b);
    println!("Key exchange successful!");
}

fn mod_exp(base: u64, exp: u64, modulus: u64) -> u64 {
    let mut result = 1;
    let mut base = base % modulus;
    let mut exp = exp;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp = exp >> 1;
        base = (base * base) % modulus;
    }
    result
}
```

- `mod_exp` 函数使用平方-乘（square-and-multiply）算法实现模幂运算以提高效率。
- 示例使用较小的数字（p=23, g=5）便于演示；实际实现需要使用更大的素数（通常为 2048 位或更高）。
- 该示例为可重复性使用了固定私钥（6 和 15）；实际实现应使用安全生成的随机私钥。
- 共享密钥可用于后续对称加密通信。
- 用于生产环境时，应使用像 `ring` 或 `openssl` 这样的库，并选择合适的参数与安全随机数生成。
