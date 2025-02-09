### Observer Pattern

The observer pattern is a behavioral design pattern where an object (the subject) maintains a list of its dependents (observers) and notifies them of any state changes, usually by calling one of their methods.

```rust
{{#include observer/src/main.rs}}
```

Observer Trait defines the update method that observers must implement. Subject Struct manages a list of observers and notifies them of changes. Concrete Observers implement the Observer trait and define the update method.

This setup allows the subject to notify all registered observers whenever a change occurs, following the Observer pattern principles.