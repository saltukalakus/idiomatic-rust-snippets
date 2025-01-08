### Diffie-Hellman Key Exchange

The Diffie-Hellman key exchange algorithm allows two parties to securely share a secret key over an insecure channel. 

### Explanation

1. **Choose Prime Numbers**: Both parties agree on a large prime number `p` and a base `g`.
2. **Generate Private Keys**: Each party generates a private key.
3. **Compute Public Keys**: Each party computes their public key using the formula `g^private_key % p`.
4. **Exchange Public Keys**: The public keys are exchanged between the parties.
5. **Compute Shared Secret**: Each party computes the shared secret using the received public key and their own private key.

Below is a simple implementation in Rust without using any external libraries.

```rust
extern crate rand;

use rand::Rng;

fn main() {
    // Step 1: Agree on a prime number p and base g
    let p: u64 = 23; // A small prime number for simplicity
    let g: u64 = 5;  // A primitive root modulo p

    // Step 2: Each party generates a private key
    let private_key_a = generate_private_key();
    let private_key_b = generate_private_key();

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
}

fn generate_private_key() -> u64 {
    rand::thread_rng().gen_range(1..100)
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

## Explanation of the Code

- `generate_private_key`: Generates a random private key.
- `mod_exp`: Computes the modular exponentiation using the formula `(base^exp) % modulus`.
- The main function demonstrates the key exchange process, where both parties compute their public keys, exchange them, and then compute the shared secret.

This example uses small numbers for simplicity. In a real-world scenario, much larger prime numbers should be used to ensure security.