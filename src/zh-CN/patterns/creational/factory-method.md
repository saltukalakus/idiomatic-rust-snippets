### Factory Method Pattern

The Factory Method provides an interface for creating objects, allowing implementations to decide which concrete type to instantiate. This decouples object creation from the code that uses the objects.

**Benefits**:
- Decouples object creation from usage
- Easy to add new types without modifying existing code
- Promotes dependency injection and testability
- Uses traits for polymorphic behavior

```rust, editable
{{#include factory-method/src/main.rs}}
```

**Key Points**:
- The example defines `Shape` trait with `draw()` method, implemented by `Circle` and `Square`
- `ShapeFactory` trait declares `create_shape()` which returns `Box<dyn Shape>`
- `CircleFactory` creates circles, `SquareFactory` creates squares - each returns the appropriate type
- In `main()`, factories create shapes without client knowing concrete types
- To add new shapes (e.g., Triangle), just implement `Shape` trait and create a `TriangleFactory`

**When to Use**:
- When the exact type to create isn't known until runtime
- To decouple object creation from business logic
- When you want to extend object creation without changing existing code
- For testability (inject mock factories)