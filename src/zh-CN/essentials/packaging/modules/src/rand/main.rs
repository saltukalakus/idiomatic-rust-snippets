extern crate rand;
use rand::Rng;

fn main() {
    // 创建一个随机数生成器
    let mut rng = rand::thread_rng();

    // 生成一个 1 到 10 之间的随机数
    let random_number: u32 = rng.gen_range(1..=10);

    println!("随机数: {}", random_number);
}