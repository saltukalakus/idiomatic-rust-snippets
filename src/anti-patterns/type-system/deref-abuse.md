### Misusing Deref Coercion

Implementing `Deref` to create inheritance-like relationships is a misuse of the trait. `Deref` is designed for smart pointers (`Box<T>`, `Rc<T>`) to transparently access their inner type. Using it for "is-a" relationships creates confusing APIs where types unexpectedly coerce and method resolution becomes unpredictable.

`Deref` should only be implemented for types that act like smart pointers. For composition, use explicit fields and methods.

```rust, editable
use std::ops::Deref;

struct User {
    name: String,
    email: String,
}

struct Admin {
    user: User,
    permissions: Vec<String>,
}

impl Deref for Admin {
    type Target = User;
    
    fn deref(&self) -> &Self::Target {
        &self.user
    }
}

impl User {
    fn display(&self) {
        println!("{} <{}>", self.name, self.email);
    }
}

fn main() {
    let admin = Admin {
        user: User {
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        },
        permissions: vec!["read".to_string(), "write".to_string()],
    };
    
    admin.display(); // Works via Deref, but confusing
    println!("Name: {}", admin.name); // Also works via Deref coercion
}
```

`Admin` derefs to `User`, creating an implicit "is-a" relationship that Rust doesn't have. This is confusing because `Admin` isn't a smart pointer to `User`. Method calls like `admin.display()` work via auto-deref, but it's not obvious. Readers must understand that `Admin` derefs to `User`. This pattern mimics inheritance, which Rust intentionally doesn't support.

The next sample uses explicit composition.

```rust, editable
struct User {
    name: String,
    email: String,
}

struct Admin {
    user: User,
    permissions: Vec<String>,
}

impl User {
    fn display(&self) {
        println!("{} <{}>", self.name, self.email);
    }
}

impl Admin {
    fn user(&self) -> &User {
        &self.user
    }
    
    fn display(&self) {
        self.user.display();
        println!("Permissions: {:?}", self.permissions);
    }
}

fn main() {
    let admin = Admin {
        user: User {
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        },
        permissions: vec!["read".to_string(), "write".to_string()],
    };
    
    admin.display(); // Clearly calls Admin's display
    println!("Name: {}", admin.user().name); // Explicit access to user data
}
```

**Key Improvements**:
- No `Deref` abuse - composition is explicit
- `Admin` provides a `user()` accessor method when needed
- `Admin::display()` can extend or wrap `User::display()` as needed
- Clear which type's methods are being called - no hidden coercion
- Follows Rust idioms - prefer composition over Deref trickery
