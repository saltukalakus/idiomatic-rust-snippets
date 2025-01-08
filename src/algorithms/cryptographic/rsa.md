### RSA Algorithm

RSA (Rivest-Shamir-Adleman) is a widely used public-key cryptosystem for secure data transmission. It is based on the mathematical difficulty of factoring the product of two large prime numbers.

### How RSA Works

1. **Key Generation**:
    - Choose two distinct large random prime numbers `p` and `q`.
    - Compute `n = p * q`. `n` is used as the modulus for both the public and private keys.
    - Compute the totient function `φ(n) = (p-1) * (q-1)`.
    - Choose an integer `e` such that `1 < e < φ(n)` and `e` is coprime with `φ(n)`. `e` is the public exponent.
    - Determine `d` as `d ≡ e^(-1) (mod φ(n))`. `d` is the private exponent.

2. **Encryption**:
    - The public key is `(e, n)`.
    - The plaintext message `m` is encrypted to ciphertext `c` using the formula: `c ≡ m^e (mod n)`.

3. **Decryption**:
    - The private key is `(d, n)`.
    - The ciphertext `c` is decrypted back to plaintext `m` using the formula: `m ≡ c^d (mod n)`.

Below is a simple example of RSA encryption and decryption in Rust:

```rust
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

This example demonstrates the basic steps of RSA key generation, encryption, and decryption. Note that in a real-world application, you should use much larger prime numbers and a cryptographic library for secure key generation and operations.