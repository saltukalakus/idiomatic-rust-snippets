### Abstract Factory Pattern

The abstract factory provides an interface for creating families of related objects without specifying their concrete types. This ensures that created objects are compatible with each other.

**Benefits**:
- Creates families of related objects that work together
- Isolates concrete classes from client code
- Easy to swap entire product families
- Ensures consistency among related products

```rust
{{#include abstract-method/src/main.rs}}
```

**Key Points**:
- The example defines `Chair` and `Sofa` traits for furniture products
- `FurnitureFactory` trait declares `create_chair()` and `create_sofa()` methods
- `ModernFurnitureFactory` creates modern-styled chair and sofa; `VictorianFurnitureFactory` creates Victorian-styled ones
- Client receives a factory and calls methods to get matching furniture without knowing concrete types
- All furniture from one factory shares the same style, ensuring visual consistency

**When to Use**:
- When you need families of related objects
- To ensure products work together (same theme/style)
- When you want to switch product families at runtime
- For UI themes, database drivers, or platform-specific components