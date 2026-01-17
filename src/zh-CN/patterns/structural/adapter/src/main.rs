// 我们想要适配的现有接口
trait Target {
    fn request(&self) -> String;
}

// 具有不同接口的被适配类
struct Adaptee;

impl Adaptee {
    fn specific_request(&self) -> String {
        "specific 请求".to_string()
    }
}

// 使 Adaptee 与 Target 兼容的适配器类
struct Adapter {
    adaptee: Adaptee,
}

impl Adapter {
    fn new(adaptee: Adaptee) -> Self {
        Adapter { adaptee }
    }
}

impl Target for Adapter {
    fn request(&self) -> String {
        // 将 Adaptee 的接口转换为目标接口
        self.adaptee.specific_request()
    }
}

fn main() {
    let adaptee = Adaptee;
    let adapter = Adapter::new(adaptee);

    // 客户端代码可以像使用 Target 一样使用适配器
    println!("适配器 请求: {}", adapter.request());
}