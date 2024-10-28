mod observer;
mod subject;
mod concrete_observer;

use subject::Subject;
use concrete_observer::ConcreteObserver;

fn main() {
    let mut subject = Subject::new();

    let observer1 = ConcreteObserver::new("Observer 1");
    let observer2 = ConcreteObserver::new("Observer 2");

    subject.add_observer(Box::new(observer1));
    subject.add_observer(Box::new(observer2));

    subject.notify_observers("Hello, observers!");
}