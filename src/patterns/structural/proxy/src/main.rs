trait Subject {
    fn request(&self) -> String;
}

struct RealSubject;

impl Subject for RealSubject {
    fn request(&self) -> String {
        "RealSubject: Handling request.".to_string()
    }
}

struct Proxy {
    real_subject: RealSubject,
}

impl Proxy {
    fn new() -> Self {
        Proxy {
            real_subject: RealSubject,
        }
    }
}

impl Subject for Proxy {
    fn request(&self) -> String {
        println!("Proxy: Logging access to RealSubject.");
        self.real_subject.request()
    }
}

fn main() {
    let proxy = Proxy::new();
    println!("{}", proxy.request());
}