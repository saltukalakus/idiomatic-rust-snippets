### Command Design Pattern

The Command Design Pattern is a behavioral design pattern that turns a request into a stand-alone object that contains all information about the request. This transformation allows for parameterization of methods with different requests, queuing of requests, and logging of the requests. It also provides support for undoable operations. <br/>

### Components of Command Pattern

1. **Command**: Declares an interface for executing an operation.
2. **ConcreteCommand**: Implements the Command interface and defines a binding between a Receiver object and an action.
3. **Client**: Creates a ConcreteCommand object and sets its receiver.
4. **Invoker**: Asks the command to carry out the request.
5. **Receiver**: Knows how to perform the operations associated with carrying out a request.

Here is an example of the Command Design Pattern:

```rust
{{#include command/src/main.rs}}
```

In this example:
- `Command` is a trait that declares the `execute` method.
- `Light` is the receiver that knows how to perform the operations.
- `TurnOnCommand` and `TurnOffCommand` are concrete commands that implement the `Command` trait.
- `RemoteControl` is the invoker that triggers the commands.

This pattern decouples the object that invokes the operation from the one that knows how to perform it.