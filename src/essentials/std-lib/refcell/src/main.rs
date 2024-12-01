use std::cell::RefCell;

fn main() {
    let data = RefCell::new(5);

    // Borrowing the value immutably
    {
        let value = data.borrow();
        println!("Value: {}", *value);
    }

    // Borrowing the value mutably
    {
        let mut value = data.borrow_mut();
        *value += 1;
    }

    // Borrowing the value immutably again to see the change
    {
        let value = data.borrow();
        println!("Updated Value: {}", *value);
    }
}