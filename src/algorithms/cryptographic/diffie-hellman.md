### Diffie-Hellman Key Exchange

The Diffie-Hellman key exchange algorithm allows two parties to securely establish a shared secret key over an insecure channel. The security relies on the difficulty of computing discrete logarithms.

**Key Exchange Steps:**
- Both parties agree on a large prime number `p` and a base `g` (a primitive root modulo p).
- Each party generates a private key independently.
- Each party computes their public key using the formula `g^private_key mod p`.
- The public keys are exchanged between the parties.
- Each party computes the shared secret using the received public key and their own private key: `public_key^private_key mod p`.

```rust
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

- The `mod_exp` function implements modular exponentiation using the square-and-multiply algorithm for efficiency.
- Small numbers (p=23, g=5) are used for demonstration; real implementations require much larger primes (typically 2048 bits or more).
- This example uses fixed private keys (6 and 15) for reproducibility; in real implementations, random keys would be generated securely.
- Party A generates private key and computes public key: `public_A = g^private_A mod p`.
- Party B generates private key and computes public key: `public_B = g^private_B mod p`.
- Both parties exchange public keys over the insecure channel (eavesdroppers can see public keys but cannot derive the private keys).
- Party A computes shared secret: `secret_A = public_B^private_A mod p`.
- Party B computes shared secret: `secret_B = public_A^private_B mod p`.
- Both shared secrets are equal due to the mathematical property: `(g^a)^b = (g^b)^a = g^(ab) mod p`.
- The assertion confirms both parties computed the same shared secret.
- The shared secret can then be used as a symmetric key for further encrypted communication.
- This example uses small numbers for clarity; production implementations should use cryptographic libraries like `ring` or `openssl` with properly sized parameters and secure random number generation.