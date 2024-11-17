# Strategy Pattern

The strategy pattern defines a family of algorithms, encapsulates each one, and makes them interchangeable. This pattern lets the algorithm vary independently from clients that use it.

## Example

Here's a simple example of the strategy pattern in Rust:

### Step 1: Define the Strategy Trait

```rust,noplaypen
{{#include strategy/src/strategy.rs}}
```

### Step 2: Implement Concrete Strategies

```rust,noplaypen
{{#include strategy/src/concrete_strategy.rs}}
```

### Step 3: Define the Context

```rust,noplaypen
{{#include strategy/src/context.rs}}
```

### Step 4: Use the Strategy Pattern

```rust,noplaypen
{{#include strategy/src/main.rs}}
```

You may find the sample project [here](https://github.com/saltukalakus/idiomatic-rust-snippets/tree/main/src/patterns/behavioral/strategy).