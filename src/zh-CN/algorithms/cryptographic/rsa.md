### RSA 算法

RSA（Rivest-Shamir-Adleman）是一种广泛使用的公钥密码体系，用于安全的数据传输。其安全性基于将两个大素数相乘所得积进行因式分解的困难性。

**注意**：该示例需要外部 crate。请在 `Cargo.toml` 中添加：
```toml
[dependencies]
num-bigint = "0.4"
num-traits = "0.2"
```

### RSA 工作原理

**密钥生成：**
- 选择两个不同的大随机素数 `p` 和 `q`。
- 计算 `n = p * q`。`n` 用作公钥和私钥的模数。
- 计算欧拉函数 `φ(n) = (p-1) * (q-1)`。
- 选择整数 `e`，满足 `1 < e < φ(n)` 且 `e` 与 `φ(n)` 互质。`e` 为公钥指数。
- 计算 `d`，满足 `d ≡ e^(-1) (mod φ(n))`。`d` 为私钥指数。

**加密：**
- 公钥为 `(e, n)`。
- 明文消息 `m` 使用公式 `c ≡ m^e (mod n)` 加密为密文 `c`。

**解密：**
- 私钥为 `(d, n)`。
- 使用公式 `m ≡ c^d (mod n)` 将密文 `c` 解密回明文 `m`。

```rust, editable
extern crate num_bigint as bigint;
extern crate num_traits;
extern crate rand;

use bigint::{BigInt, ToBigInt};
use num_traits::{One, Zero};
use rand::Rng;

fn main() {
     // Generate two large prime numbers (for simplicity, small primes are used here)
     let p = 61.to_bigint().unwrap();
     let q = 53.to_bigint().unwrap();
     
     // Compute n = p * q
     let n = &p * &q;
     
     // Compute φ(n) = (p-1) * (q-1)
     let phi = (&p - 1) * (&q - 1);
     
     // Choose e such that 1 < e < φ(n) and e is coprime with φ(n)
     let e = 17.to_bigint().unwrap();
     
     // Compute d as d ≡ e^(-1) (mod φ(n))
     let d = mod_inverse(&e, &phi).unwrap();
     
     // Public key (e, n) and private key (d, n)
     println!("Public Key: (e: {}, n: {})", e, n);
     println!("Private Key: (d: {}, n: {})", d, n);
     
     // Encrypt a message
     let message = 42.to_bigint().unwrap();
     let ciphertext = mod_exp(&message, &e, &n);
     println!("Encrypted message: {}", ciphertext);
     
     // Decrypt the message
     let decrypted_message = mod_exp(&ciphertext, &d, &n);
     println!("Decrypted message: {}", decrypted_message);
}

// Function to compute modular exponentiation
fn mod_exp(base: &BigInt, exp: &BigInt, modulus: &BigInt) -> BigInt {
     base.modpow(exp, modulus)
}

// Function to compute modular inverse
fn mod_inverse(a: &BigInt, m: &BigInt) -> Option<BigInt> {
     let (g, x, _) = extended_gcd(a, m);
     if g.is_one() {
          Some((x % m + m) % m)
     } else {
          None
     }
}

// Function to compute extended GCD
fn extended_gcd(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
     if b.is_zero() {
          (a.clone(), BigInt::one(), BigInt::zero())
     } else {
          let (g, x, y) = extended_gcd(b, &(a % b));
          (g, y.clone(), x - (a / b) * y)
     }
}
```

- 该示例使用 `num_bigint` crate 进行任意精度算术，以便在 RSA 操作中处理大整数。
- 示例中使用了较小的素数（61 和 53）以便演示；真实环境中需要更大的素数（通常为 1024-4096 位）。
- 公钥指数 `e` 常选为 17 或 65537，以兼顾效率与安全性。
- `mod_exp` 函数使用模幂运算（`modpow`）高效计算大指数模运算。
- `mod_inverse` 函数使用扩展欧几里得算法计算 e 关于 φ(n) 的乘法逆元。
- `extended_gcd` 函数实现扩展欧几里得算法以求贝祖等式的系数。
- 示例对消息 42 进行加密并成功解密回原始值。
- 在生产环境中，请使用像 `rsa` 或 `openssl` 这样的经过充分测试的密码库，而非自行实现 RSA。
