use std::marker::PhantomData;

// State marker types (zero-sized types)
struct Draft;
struct Review;
struct Published;

// Generic document with state as type parameter
struct Document<State> {
    content: String,
    _state: PhantomData<State>,
}

// Methods available only in Draft state
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
    
    // Transition from Draft -> Review (consumes self, returns new type)
    fn submit_for_review(self) -> Document<Review> {
        println!("Document submitted for review");
        Document {
            content: self.content,
            _state: PhantomData,
        }
    }
}

// Methods available only in Review state
impl Document<Review> {
    // Can go back to Draft
    fn request_changes(self, feedback: &str) -> Document<Draft> {
        println!("Changes requested: {}", feedback);
        Document {
            content: self.content,
            _state: PhantomData,
        }
    }
    
    // Or move forward to Published
    fn approve(self) -> Document<Published> {
        println!("Document approved and published");
        Document {
            content: self.content,
            _state: PhantomData,
        }
    }
}

// Methods available only in Published state
impl Document<Published> {
    fn read(&self) -> &str {
        &self.content
    }
    
    // Can create a new draft from published (for revisions)
    #[allow(dead_code)]
    fn create_revision(self) -> Document<Draft> {
        println!("Creating revision from published document");
        Document {
            content: self.content,
            _state: PhantomData,
        }
    }
}

// Methods available in ALL states
impl<State> Document<State> {
    fn content_length(&self) -> usize {
        self.content.len()
    }
}

// Another example: Builder with required fields
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

// build() is ONLY available when both host AND port are set
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
    println!("=== Document State Machine ===\n");
    
    // Create a draft and write content
    let mut doc = Document::<Draft>::new();
    doc.write("Hello, World!");
    println!("Draft length: {}", doc.content_length());
    
    // Submit for review (Draft -> Review)
    let doc = doc.submit_for_review();
    
    // Request changes (Review -> Draft)
    let mut doc = doc.request_changes("Add more content");
    doc.write(" More content added.");
    
    // Submit again and approve (Draft -> Review -> Published)
    let doc = doc.submit_for_review();
    let doc = doc.approve();
    
    // Now we can read the published document
    println!("Published content: {}", doc.read());
    
    // These would be COMPILE ERRORS:
    // doc.write("Can't write to published");  // Error: no method `write` for Document<Published>
    // doc.approve();                          // Error: no method `approve` for Document<Published>
    
    println!("\n=== Type-Safe Builder ===\n");
    
    // Must set both host and port before calling build()
    let connection = ConnectionBuilder::new()
        .host("localhost")
        .port(8080)
        .timeout(60)
        .build();
    
    println!("{}", connection);
    
    // This would be a COMPILE ERROR:
    // let incomplete = ConnectionBuilder::new()
    //     .host("localhost")
    //     .build();  // Error: no method `build` for ConnectionBuilder<Set, Unset>
}
