### Mediator Pattern

The mediator centralizes complex communications between objects. Instead of objects referring to each other directly, they communicate through a mediator.

**Benefits**:
- Reduces coupling between components
- Centralizes interaction logic in one place
- Components can be reused independently
- Easy to understand object interactions

```rust
{{#include mediator/src/main.rs}}
```

**Key Points**:
- The example shows `Component1` and `Component2` communicating through `ConcreteMediator`
- Each component holds `Arc<Mutex<ConcreteMediator>>` reference to the mediator
- When `Component1` calls `operation_a()`, it notifies mediator which then updates `Component2`
- When `Component2` calls `operation_b()`, mediator handles it and updates `Component1`
- Components never directly reference each other - all communication flows through mediator

**When to Use**:
- Complex communication between multiple objects
- Objects are tightly coupled and hard to reuse
- UI components that interact (dialogs, forms)
- Chat systems, air traffic control, or coordination logic
