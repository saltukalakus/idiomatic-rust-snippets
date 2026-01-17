use std::sync::{Arc, Mutex, OnceLock};
use std::thread;

// 使用 OnceLock 的全局状态 - 惯用且安全
struct AppConfig {
    data: i32,
}

impl AppConfig {
    fn new() -> Self {
        AppConfig { data: 0 }
    }

    fn increment(&mut self) {
        self.data += 1;
    }

    fn get_data(&self) -> i32 {
        self.data
    }
}

// 使用 OnceLock 的安全单例 - 无需 unsafe 代码
fn get_config() -> &'static Arc<Mutex<AppConfig>> {
    static CONFIG: OnceLock<Arc<Mutex<AppConfig>>> = OnceLock::new();
    CONFIG.get_or_init(|| Arc::new(Mutex::new(AppConfig::new())))
}

fn main() {
    // 获取单例实例
    let config = get_config();

    // 在主线程中使用
    {
        let mut instance = config.lock().unwrap();
        instance.increment();
        println!("主 thread data: {}", instance.get_data());
    }

    // 为线程克隆 Arc - 都引用相同的单例
    let config1 = Arc::clone(config);
    let config2 = Arc::clone(config);

    let handle1 = thread::spawn(move || {
        let mut instance = config1.lock().unwrap();
        instance.increment();
        println!("Thread 1 data: {}", instance.get_data());
    });

    let handle2 = thread::spawn(move || {
        let mut instance = config2.lock().unwrap();
        instance.increment();
        println!("Thread 2 data: {}", instance.get_data());
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    // 验证最终状态
    {
        let instance = config.lock().unwrap();
        println!("最终数据: {}", instance.get_data());
    }
}