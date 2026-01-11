### RAII (Resource Acquisition Is Initialization)

RAII is a fundamental pattern in Rust where resources (memory, files, locks, connections) are automatically cleaned up when their owner goes out of scope. This is implemented through Rust's ownership system and the `Drop` trait.

**Benefits**:
- No resource leaks - cleanup is guaranteed
- Exception-safe - cleanup happens even if panics occur
- No garbage collector needed
- Explicit resource lifetime tied to scope

```rust
use std::fs::File;
use std::io::{Write, BufWriter};

// Standard library types implement RAII automatically
fn write_to_file() -> std::io::Result<()> {
    let file = File::create("example.txt")?;
    let mut writer = BufWriter::new(file);
    
    writer.write_all(b"Hello, RAII!")?;
    
    // File and buffer are automatically closed and flushed when they go out of scope
    Ok(())
}

// Custom RAII: Implementing Drop for automatic cleanup
struct DatabaseConnection {
    connection_id: u32,
    is_connected: bool,
}

impl DatabaseConnection {
    fn new(id: u32) -> Self {
        println!("Opening database connection {}", id);
        DatabaseConnection {
            connection_id: id,
            is_connected: true,
        }
    }
    
    fn query(&self, sql: &str) -> String {
        if self.is_connected {
            format!("Result from connection {}: executed '{}'", self.connection_id, sql)
        } else {
            "Not connected".to_string()
        }
    }
}

impl Drop for DatabaseConnection {
    fn drop(&mut self) {
        println!("Closing database connection {}", self.connection_id);
        self.is_connected = false;
        // In real code: close connection, release resources, etc.
    }
}

// RAII Guard pattern for scoped behavior
struct MutexGuard<'a, T> {
    data: &'a mut T,
    // In real implementation: reference to the mutex
}

impl<T> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        println!("Releasing lock");
        // In real implementation: release the mutex
    }
}

fn main() {
    println!("=== Database Connection RAII ===");
    {
        let conn = DatabaseConnection::new(1);
        println!("{}", conn.query("SELECT * FROM users"));
        // Connection automatically closed here when conn goes out of scope
    }
    println!("Connection is now closed\n");
    
    println!("=== Nested RAII ===");
    {
        let conn1 = DatabaseConnection::new(1);
        {
            let conn2 = DatabaseConnection::new(2);
            println!("{}", conn1.query("Query 1"));
            println!("{}", conn2.query("Query 2"));
            // conn2 dropped here (LIFO order)
        }
        println!("{}", conn1.query("Query 3"));
        // conn1 dropped here
    }
    
    println!("\n=== RAII even with early returns ===");
    fn might_fail(should_fail: bool) -> Result<(), &'static str> {
        let _conn = DatabaseConnection::new(99);
        
        if should_fail {
            return Err("Something went wrong");
            // Connection STILL gets closed!
        }
        
        println!("Success path");
        Ok(())
    }
    
    let _ = might_fail(true);  // Connection closed even on error
    let _ = might_fail(false); // Connection closed normally
}
```

**Key Points**:
- Resources are acquired in constructors (`new`)
- Resources are released in `Drop::drop()`
- Drop is called automatically when value goes out of scope
- Drop order is LIFO (Last In, First Out) for nested resources
- Works correctly with `?` operator and early returns

**Common RAII Types in std**:
- `File` - closes file handle on drop
- `MutexGuard` - releases lock on drop
- `Box`, `Vec`, `String` - deallocate memory on drop
- `Rc`, `Arc` - decrement reference count on drop
- `JoinHandle` - can be used to ensure thread cleanup

**When to Use**:
- Whenever you have a resource that needs cleanup
- File handles, network connections, locks
- Temporary state that must be restored
- Any acquire/release pattern
