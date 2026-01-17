use std::cell::RefCell;

fn main() {
    let data = RefCell::new(5);

    // 不可变地借用值
    {
        let value = data.borrow();
        println!("值: {}", *value);
    }

    // 可变地借用值
    {
        let mut value = data.borrow_mut();
        *value += 1;
    }

    // 再次不可变地借用值以查看更改
    {
        let value = data.borrow();
        println!("更新后的值: {}", *value);
    }
}