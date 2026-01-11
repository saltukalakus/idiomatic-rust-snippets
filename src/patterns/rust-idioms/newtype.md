### Newtype Pattern

The Newtype pattern wraps an existing type in a tuple struct with a single field. This provides type safety, encapsulation, and the ability to implement traits on the wrapper type without affecting the original type.

**Benefits**:
- Zero-cost abstraction (compiled away at runtime)
- Type safety - distinct types even if they wrap the same underlying type
- Implement external traits on external types (orphan rule workaround)
- Hide implementation details
- Add semantic meaning to primitive types

```rust
// Problem: Easy to mix up parameters with the same underlying type
fn process_user_bad(user_id: u64, account_id: u64) {
    println!("Processing user {} in account {}", user_id, account_id);
}

// Newtype pattern: Wrap types for safety
struct UserId(u64);
struct AccountId(u64);

impl UserId {
    fn new(id: u64) -> Self {
        UserId(id)
    }
    
    fn value(&self) -> u64 {
        self.0
    }
}

impl AccountId {
    fn new(id: u64) -> Self {
        AccountId(id)
    }
    
    fn value(&self) -> u64 {
        self.0
    }
}

// Now the compiler prevents mixing up the parameters
fn process_user(user_id: UserId, account_id: AccountId) {
    println!("Processing user {} in account {}", user_id.value(), account_id.value());
}

fn main() {
    let user = UserId::new(42);
    let account = AccountId::new(100);
    
    process_user(user, account);  // Compiles
    
    // process_user(account, user);  // Compile error! Types don't match
    
    // Common use: Implementing external traits on external types
    struct Meters(f64);
    struct Kilometers(f64);
    
    impl From<Kilometers> for Meters {
        fn from(km: Kilometers) -> Self {
            Meters(km.0 * 1000.0)
        }
    }
    
    let distance_km = Kilometers(5.0);
    let distance_m: Meters = distance_km.into();
    println!("5 km = {} meters", distance_m.0);
}
```

**Key Points**:
- The wrapper type is compiled away - zero runtime overhead
- Prevents accidentally mixing up values of the same underlying type
- Allows implementing traits on external types (e.g., implementing `Display` for `Vec<T>`)
- Use `#[repr(transparent)]` if you need guaranteed same memory layout as the inner type

**When to Use**:
- When you have multiple parameters of the same type (like `u64`)
- When you want to add semantic meaning to a type
- When you need to implement external traits on external types
- When you want to restrict what operations are valid on a type
