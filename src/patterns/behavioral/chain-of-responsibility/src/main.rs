trait Handler {
    fn set_next(&mut self, next: Box<dyn Handler>);
    fn handle(&self, request: &str);
}

struct BaseHandler {
    next: Option<Box<dyn Handler>>,
}

impl BaseHandler {
    fn new() -> Self {
        BaseHandler { next: None }
    }
}

impl Handler for BaseHandler {
    fn set_next(&mut self, next: Box<dyn Handler>) {
        self.next = Some(next);
    }

    fn handle(&self, request: &str) {
        if let Some(ref next) = self.next {
            next.handle(request);
        }
    }
}

struct ConcreteHandlerA {
    next: Option<Box<dyn Handler>>,
}

impl Handler for ConcreteHandlerA {
    fn set_next(&mut self, next: Box<dyn Handler>) {
        self.next = Some(next);
    }

    fn handle(&self, request: &str) {
        if request == "A" {
            println!("ConcreteHandlerA handled request: {}", request);
        } else {
            println!("ConcreteHandlerA passed the request");
            if let Some(ref next) = self.next {
                next.handle(request);
            }
        }
    }
}

struct ConcreteHandlerB {
    next: Option<Box<dyn Handler>>,
}

impl Handler for ConcreteHandlerB {
    fn set_next(&mut self, next: Box<dyn Handler>) {
        self.next = Some(next);
    }

    fn handle(&self, request: &str) {
        if request == "B" {
            println!("ConcreteHandlerB handled request: {}", request);
        } else {
            println!("ConcreteHandlerB passed the request");
            if let Some(ref next) = self.next {
                next.handle(request);
            } 
        }
    }
}


fn main() {
    let mut handler = BaseHandler::new();
    let mut handler_b = Box::new(ConcreteHandlerB { next: None });
    let handler_a = Box::new(ConcreteHandlerA { next: None });

    handler_b.set_next(handler_a);
    handler.set_next(handler_b);

    println!("Handle request A");
    handler.handle("A");
    println!("Handle request B");
    handler.handle("B");
    println!("Handle request C");
    handler.handle("C");
}