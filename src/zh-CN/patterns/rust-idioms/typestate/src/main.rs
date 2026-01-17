use std::marker::PhantomData;

// 状态 marker types (zero-sized types)
struct Draft;
struct Review;
struct Published;

// 带有状态作为类型参数的通用文档
struct Document<State> {
    content: String,
    _state: PhantomData<State>,
}

// 仅在草稿状态下可用的方法
impl Document<Draft> {
    fn new() -> Self {
        Document {
            content: String::new(),
            _state: PhantomData,
        }
    }
    
    fn write(&mut self, text: &str) {
        self.content.push_str(text);
    }
    
    // 从草稿转换到审核（消耗 self，返回新类型）
    fn submit_for_review(self) -> Document<Review> {
        println!("文档已提交审核");
        Document {
            content: self.content,
            _state: PhantomData,
        }
    }
}

// 仅在审核状态下可用的方法
impl Document<Review> {
    // Can go back to Draft
    fn request_changes(self, feedback: &str) -> Document<Draft> {
        println!("请求更改: {}", feedback);
        Document {
            content: self.content,
            _state: PhantomData,
        }
    }
    
    // Or move forward to Published
    fn approve(self) -> Document<Published> {
        println!("文档已审批并发布");
        Document {
            content: self.content,
            _state: PhantomData,
        }
    }
}

// 仅在已发布状态下可用的方法
impl Document<Published> {
    fn read(&self) -> &str {
        &self.content
    }
    
    // 可以从已发布创建新的草稿（用于修订）
    #[allow(dead_code)]
    fn create_revision(self) -> Document<Draft> {
        println!("从已发布文档创建修订版");
        Document {
            content: self.content,
            _state: PhantomData,
        }
    }
}

// 在所有状态下可用的方法
impl<State> Document<State> {
    fn content_length(&self) -> usize {
        self.content.len()
    }
}

// 另一个示例：具有必填字段的构建器
struct Unset;
struct Set;

struct ConnectionBuilder<HostState, PortState> {
    host: Option<String>,
    port: Option<u16>,
    timeout: u32,
    _host_state: PhantomData<HostState>,
    _port_state: PhantomData<PortState>,
}

impl ConnectionBuilder<Unset, Unset> {
    fn new() -> Self {
        ConnectionBuilder {
            host: None,
            port: None,
            timeout: 30,
            _host_state: PhantomData,
            _port_state: PhantomData,
        }
    }
}

impl<PortState> ConnectionBuilder<Unset, PortState> {
    fn host(self, host: &str) -> ConnectionBuilder<Set, PortState> {
        ConnectionBuilder {
            host: Some(host.to_string()),
            port: self.port,
            timeout: self.timeout,
            _host_state: PhantomData,
            _port_state: PhantomData,
        }
    }
}

impl<HostState> ConnectionBuilder<HostState, Unset> {
    fn port(self, port: u16) -> ConnectionBuilder<HostState, Set> {
        ConnectionBuilder {
            host: self.host,
            port: Some(port),
            timeout: self.timeout,
            _host_state: PhantomData,
            _port_state: PhantomData,
        }
    }
}

impl<HostState, PortState> ConnectionBuilder<HostState, PortState> {
    fn timeout(mut self, timeout: u32) -> Self {
        self.timeout = timeout;
        self
    }
}

// build() 仅在 host 和 port 都设置时可用
impl ConnectionBuilder<Set, Set> {
    fn build(self) -> String {
        format!(
            "Connection to {}:{} (timeout: {}s)",
            self.host.unwrap(),
            self.port.unwrap(),
            self.timeout
        )
    }
}

fn main() {
    println!("=== Document 状态 Machine ===\n");
    
    // 创建草稿并编写内容
    let mut doc = Document::<Draft>::new();
    doc.write("你好, 世界!");
    println!("草稿长度: {}", doc.content_length());
    
    // 提交审核（草稿 -> 审核）
    let doc = doc.submit_for_review();
    
    // 请求 changes (Review -> Draft)
    let mut doc = doc.request_changes("Add more content");
    doc.write(" More content added.");
    
    // 再次提交并批准（草稿 -> 审核 -> 已发布）
    let doc = doc.submit_for_review();
    let doc = doc.approve();
    
    // 现在我们可以读取已发布的文档
    println!("已发布内容: {}", doc.read());
    
    // 这些将是编译错误：
    // doc.write("Can't write to published");  // 错误: no method `write` for Document<Published>
    // doc.approve();                          // 错误: no method `approve` for Document<Published>
    
    println!("\n=== Type-Safe 构建器 ===\n");
    
    // Must set both host and port before 调用 build()
    let connection = ConnectionBuilder::new()
        .host("localhost")
        .port(8080)
        .timeout(60)
        .build();
    
    println!("{}", connection);
    
    // 这将是一个编译错误：
    // let incomplete = ConnectionBuilder::new()
    //     .host("localhost")
    //     .build();  // 错误: no method `build` for ConnectionBuilder<Set, Unset>
}
