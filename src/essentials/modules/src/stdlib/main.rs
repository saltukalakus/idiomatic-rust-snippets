use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // Get the current system time
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("Current time since UNIX EPOCH: {} seconds", n.as_secs()),
        Err(_) => println!("SystemTime before UNIX EPOCH!"),
    }
}