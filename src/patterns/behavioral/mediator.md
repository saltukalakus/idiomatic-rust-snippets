### Mediator Pattern

The mediator pattern is a behavioral design pattern that defines an object that encapsulates how a set of objects interact. This pattern promotes loose coupling by keeping objects from referring to each other explicitly and allows their interaction to be varied independently. The pattern centralizes the control logic that would otherwise be distributed among several objects.

```rust
{{#include mediator/src/main.rs}}
```

`ConcreteMediator` coordinates the interactions between `Component1` and `Component2`. Each component notifies the mediator when an event occurs, and the mediator decides how to handle the event and which components to notify.

