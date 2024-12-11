### Observer Pattern

The Observer pattern is a behavioral design pattern where an object (the subject) maintains a list of its dependents (observers) and notifies them of any state changes, usually by calling one of their methods.

### Step-by-Step Plan

1- Define the Observer trait: Create a trait that observers must implement.<br/>
2- Define the Subject struct: Create a struct that maintains a list of observers and notifies them of changes.<br/>
3- Implement the Observer trait for concrete observers: Create concrete observer structs that implement the Observer trait.<br/>
4- Main function: Create a subject, add observers, and notify them of changes.<br/>

```rust
{{#include observer/src/main.rs}}
```

In this example:

* Observer Trait: Defines the update method that observers must implement.
* Subject Struct: Manages a list of observers and notifies them of changes.
* Concrete Observers: Implement the Observer trait and define the update method.
* Main Function: Demonstrates the pattern by creating a subject, adding observers, and notifying them.

This setup allows the subject to notify all registered observers whenever a change occurs, following the Observer pattern principles.