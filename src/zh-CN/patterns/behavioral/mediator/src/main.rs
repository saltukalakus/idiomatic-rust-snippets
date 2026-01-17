use std::cell::RefCell;
use std::rc::Rc;

trait Mediator {
    fn notify(&self, sender: &str, event: &str);
}

struct ConcreteMediator {
    component1: Rc<RefCell<Component1>>,
    component2: Rc<RefCell<Component2>>,
}

impl ConcreteMediator {
    fn new(component1: Rc<RefCell<Component1>>, component2: Rc<RefCell<Component2>>) -> Self {
        ConcreteMediator { component1, component2 }
    }
}

impl Mediator for ConcreteMediator {
    fn notify(&self, sender: &str, event: &str) {
        if event == "A" {
            println!("中介者响应 A 并触发以下操作:");
            self.component2.borrow().do_c();
        } else if event == "D" {
            println!("中介者响应 D 并触发以下操作:");
            self.component1.borrow().do_b();
            self.component2.borrow().do_c();
        }
    }
}

struct Component1 {
    mediator: Option<Rc<RefCell<ConcreteMediator>>>,
}

impl Component1 {
    fn new() -> Self {
        Component1 { mediator: None }
    }

    fn set_mediator(&mut self, mediator: Rc<RefCell<ConcreteMediator>>) {
        self.mediator = Some(mediator);
    }

    fn do_a(&self) {
        println!("组件 1 执行 A。");
        if let Some(ref mediator) = self.mediator {
            mediator.borrow().notify("Component1", "A");
        }
    }

    fn do_b(&self) {
        println!("组件 1 执行 B。");
        if let Some(ref mediator) = self.mediator {
            mediator.borrow().notify("Component1", "B");
        }
    }
}

struct Component2 {
    mediator: Option<Rc<RefCell<ConcreteMediator>>>,
}

impl Component2 {
    fn new() -> Self {
        Component2 { mediator: None }
    }

    fn set_mediator(&mut self, mediator: Rc<RefCell<ConcreteMediator>>) {
        self.mediator = Some(mediator);
    }

    fn do_c(&self) {
        println!("组件 2 执行 C。");
        if let Some(ref mediator) = self.mediator {
            mediator.borrow().notify("Component2", "C");
        }
    }

    fn do_d(&self) {
        println!("组件 2 执行 D。");
        if let Some(ref mediator) = self.mediator {
            mediator.borrow().notify("Component2", "D");
        }
    }
}

fn main() {
    let component1 = Rc::new(RefCell::new(Component1::new()));
    let component2 = Rc::new(RefCell::new(Component2::new()));

    let mediator = Rc::new(RefCell::new(ConcreteMediator::new(
        Rc::clone(&component1),
        Rc::clone(&component2),
    ))) as Rc<RefCell<ConcreteMediator>>;

    component1.borrow_mut().set_mediator(Rc::clone(&mediator));
    component2.borrow_mut().set_mediator(Rc::clone(&mediator));

    println!("客户端触发操作 A。");
    component1.borrow().do_a();

    println!("\n客户端触发操作 D。");
    component2.borrow().do_d();
}