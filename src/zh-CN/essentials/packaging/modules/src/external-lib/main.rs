use regex::Regex;

fn main() {
    // 创建一个正则表达式模式
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();

    // 测试模式是否匹配字符串
    let date = "2023-10-05";
    if re.is_match(date) {
        println!("日期 {} 的格式正确。", date);
    } else {
        println!("日期 {} 的格式不正确。", date);
    }
}