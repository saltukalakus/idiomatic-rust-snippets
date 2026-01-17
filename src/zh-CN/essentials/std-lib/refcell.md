### RefCell - 内部可变性（Interior Mutability）

`RefCell<T>` 提供在运行时可变借用检查的能力，允许在编译时不可变的上下文中进行可变访问（仅限单线程）。常与 `Rc<T>` 组合使用。

```rust, editable
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(5);

    // 可变借用
    *data.borrow_mut() += 1;
    println!("{}", data.borrow());
}
```

`RefCell` 在借用规则被违反时会在运行时 panic（例如同时存在多个可变借用），因此要小心使用。典型模式是 `Rc<RefCell<T>>` 来在单线程中实现可变共享。

```rust, editable
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let shared = Rc::new(RefCell::new(vec![1, 2, 3]));
    let a = Rc::clone(&shared);
    let b = Rc::clone(&shared);

    a.borrow_mut().push(4);
    println!("{:?}", b.borrow());
}
```
