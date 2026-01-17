### 标准库

本节解释一些常用标准库 API 及其在日常编程中的用法。目的不是覆盖全部 API，而是提供常用功能的实用示例。

1- [Result 类型](./result.md)：Rust 处理可能失败操作的主要机制，带有显式错误信息。

2- [Option 枚举](./option.md)：表示一个值可能存在（`Some`）或不存在（`None`）的枚举，避免空指针问题。

3- [String 与 &str](./string-vs-str.md)：理解拥有型 `String` 与借用型 `&str` 之间的重要区别。

4- [Vec](./vec.md)：可增长的数组类型，用于在运行时可增长或缩小的动态集合。

5- [HashMap](./hashmap.md)：用于按键高效查找的键值对集合。

6- [Box](./box.md)：在堆上分配的智能指针，适用于大型数据、特质对象及递归类型。

7- [Rc](./rc.md)：适用于单线程场景的引用计数智能指针，用于共享所有权。

8- [Arc 与 Mutex](./arc-mutex.md)：用于并发编程的线程安全引用计数及互斥锁。

9- [RefCell](./refcell.md)：允许在运行时检查可变借用的内部可变性模式。

10- [将 `Self` 作为类型](./self-as-type.md)：在特质实现中将 `Self` 作为类型别名使用。

11- [`self` 与 `Self` 的区别](./self-vs-self.md)：理解小写 `self` 与大写 `Self` 之间的区别。

12- [`self` 关键字](./self.md)：方法定义与实现中的 `self` 关键字用法。

13- [Some 变体](./some.md)：`Option` 枚举的 `Some` 变体用于包装值。

14- [new 约定](./new.md)：Rust 中常用的构造函数约定。

15- [Derive(Debug)](./drive(debug).md)：为自定义类型自动实现 `Debug` 特质。

16- [Impl](./imp.md)：为类型实现方法与关联函数。