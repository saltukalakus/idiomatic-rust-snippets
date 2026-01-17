use std::fs::File;
use std::io::{Write, BufWriter};

// 标准库类型自动实现 RAII
#[allow(dead_code)]
fn write_to_file() -> std::io::Result<()> {
    let file = File::create("示例.txt")?;
    let mut writer = BufWriter::new(file);
    
    writer.write_all(b"你好, RAII!")?;
    
    // 文件和缓冲区在离开作用域时自动关闭和刷新
    Ok(())
}

// 自定义 RAII：实现 Drop 以进行自动清理
struct DatabaseConnection {
    connection_id: u32,
    is_connected: bool,
}

impl DatabaseConnection {
    fn new(id: u32) -> Self {
        println!("打开数据库连接 {}", id);
        DatabaseConnection {
            connection_id: id,
            is_connected: true,
        }
    }
    
    fn query(&self, sql: &str) -> String {
        if self.is_connected {
            format!("来自连接 {} 的结果: 执行 '{}'", self.connection_id, sql)
        } else {
            "未连接".to_string()
        }
    }
}

impl Drop for DatabaseConnection {
    fn drop(&mut self) {
        println!("关闭数据库连接 {}", self.connection_id);
        self.is_connected = false;
        // 在实际代码中：关闭连接、释放资源等
    }
}

// 用于作用域行为的 RAII 守卫模式
#[allow(dead_code)]
struct MutexGuard<'a, T> {
    data: &'a mut T,
    // 在实际实现中：对互斥锁的引用
}

impl<T> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        println!("释放锁");
        // 在实际实现中：释放互斥锁
    }
}

fn 主() {
    println!("=== Database Connection RAII ===");
    {
        let conn = DatabaseConnection::new(1);
        println!("{}", conn.query("SELECT * FROM users"));
        // 当 conn 离开作用域时，连接在此处自动关闭
    }
    println!("连接现在已关闭\n");
    
    println!("=== Nested RAII ===");
    {
        let conn1 = DatabaseConnection::new(1);
        {
            let conn2 = DatabaseConnection::new(2);
            println!("{}", conn1.query("Query 1"));
            println!("{}", conn2.query("Query 2"));
            // conn2 dropped here (LIFO order)
        }
        println!("{}", conn1.query("Query 3"));
        // conn1 dropped here
    }
    
    println!("\n=== RAII even with early returns ===");
    fn might_fail(should_fail: bool) -> 结果<(), &'static str> {
        let _conn = DatabaseConnection::new(99);
        
        if should_fail {
            return Err("Something went wrong");
            // 连接仍然会被关闭！
        }
        
        println!("成功 path");
        Ok(())
    }
    
    let _ = might_fail(true);  // 即使出错连接也会关闭
    let _ = might_fail(false); // 连接正常关闭
}
