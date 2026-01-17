// 问题：容易混淆具有相同底层类型的参数
#[allow(dead_code)]
fn process_user_bad(user_id: u64, account_id: u64) {
    println!("处理 user {} in account {}", user_id, account_id);
}

// 新类型模式：为安全性包装类型
struct UserId(u64);
struct AccountId(u64);

impl UserId {
    fn new(id: u64) -> Self {
        UserId(id)
    }
    
    fn value(&self) -> u64 {
        self.0
    }
}

impl AccountId {
    fn new(id: u64) -> Self {
        AccountId(id)
    }
    
    fn value(&self) -> u64 {
        self.0
    }
}

// 现在编译器防止混淆参数
fn process_user(user_id: UserId, account_id: AccountId) {
    println!("处理 user {} in account {}", user_id.value(), account_id.value());
}

fn main() {
    let user = UserId::new(42);
    let account = AccountId::new(100);
    
    process_user(user, account);  // 编译通过
    
    // process_user(account, user);  // 编译错误！类型不匹配
    
    // 常见用途：在外部类型上实现外部 trait
    struct Meters(f64);
    struct Kilometers(f64);
    
    impl From<Kilometers> for Meters {
        fn from(km: Kilometers) -> Self {
            Meters(km.0 * 1000.0)
        }
    }
    
    let distance_km = Kilometers(5.0);
    let distance_m: Meters = distance_km.into();
    println!("5 km = {} meters", distance_m.0);
}
