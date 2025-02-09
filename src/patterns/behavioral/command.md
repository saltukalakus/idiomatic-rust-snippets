### Command Pattern

The command pattern is a behavioral design pattern that turns a request into a stand-alone object that contains all information about the request. This transformation allows for parameterization of methods with different requests, queuing of requests, and logging of the requests. It also provides support for undoable operations. <br/>

```rust
{{#include command/src/main.rs}}
```

- `Command` is a trait that declares the `execute` method.
- `Light` is the receiver that knows how to perform the operations.
- `TurnOnCommand` and `TurnOffCommand` are concrete commands that implement the `Command` trait.
- `RemoteControl` is the invoker that triggers the commands.

This pattern decouples the object that invokes the operation from the one that knows how to perform it.