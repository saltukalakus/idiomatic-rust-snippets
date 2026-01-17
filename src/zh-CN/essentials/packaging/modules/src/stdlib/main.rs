use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // 获取当前系统时间
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("自 UNIX 纪元以来的当前时间: {} 秒", n.as_secs()),
        Err(_) => println!("系统时间早于 UNIX 纪元!"),
    }
}