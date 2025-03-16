### Factory Method Pattern

The Factory Method is a creational design pattern that provides an interface for creating objects in a superclass but allows subclasses to alter the type of objects that will be created.

Tthe Factory Method pattern can be implemented using traits and structs. The trait defines the method for creating objects, and the structs implement this trait to create specific types of objects.

In this example we have a `Shape` trait and two structs `Circle` and `Square` that implement this trait. We will create a `ShapeFactory` trait with a method `create_shape` and two factories `CircleFactory` and `SquareFactory` that implement this trait.

```rust
{{#include factory-method/src/main.rs}}
```

## Output

```
Drawing a Circle
Drawing a Square
```

- The `ShapeFactory` trait defines the `create_shape` method, and the `CircleFactory` and `SquareFactory` structs implement this method to create specific shapes. This allows for flexibility in creating different types of shapes without changing the client code.