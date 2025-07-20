use std::future::Future;
use std::net::{AddrParseError, SocketAddr};
use std::vec;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::task;
use std::fmt;

// FIXED: Custom error trait that properly extends std::error::Error and ensures Send + Sync
pub trait MyError: std::error::Error + Send + Sync {
    fn as_any(&self) -> &dyn std::any::Any;
}

// FIXED: Blanket implementation for any type that implements Error + Send + Sync
impl<T> MyError for T 
where 
    T: std::error::Error + Send + Sync + 'static 
{
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

// ADDED: Custom error type for server-specific errors
#[derive(Debug)]
pub enum ServerError {
    AddrParse(AddrParseError),
    Io(std::io::Error),
    RequestParse(String),
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerError::AddrParse(e) => write!(f, "Address parse error: {}", e),
            ServerError::Io(e) => write!(f, "IO error: {}", e),
            ServerError::RequestParse(msg) => write!(f, "Request parse error: {}", msg),
        }
    }
}

impl std::error::Error for ServerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ServerError::AddrParse(e) => Some(e),
            ServerError::Io(e) => Some(e),
            ServerError::RequestParse(_) => None,
        }
    }
}

impl From<AddrParseError> for ServerError {
    fn from(err: AddrParseError) -> Self {
        ServerError::AddrParse(err)
    }
}

impl From<std::io::Error> for ServerError {
    fn from(err: std::io::Error) -> Self {
        ServerError::Io(err)
    }
}

// Simple request and response types
#[derive(Debug, Clone)]
pub struct HttpRequest {
    path: String,
    method: String,
}

impl HttpRequest {
    pub fn new(path: String, method: String) -> Self {
        Self { path, method}
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn method(&self) -> &str {
        &self.method
    }
}

#[derive(Debug)]
pub struct HttpResponse {
    status: u16,
    body: String,
    headers: Vec<(String, String)>,
}

impl HttpResponse {
    pub fn ok(body: &str) -> Self {
        Self {
            status: 200,
            body: body.to_string(),
            headers: vec![]
        }
    }

    pub fn not_found() -> Self {
        Self {
            status: 404,
            body: "Not Found".to_string(),
            headers: vec![]
        }
    }

    pub fn set_header(&mut self, key: &str, value: &str) {
        self.headers.push((key.to_string(), value.to_string()))
    }

    pub fn to_http_string(&self) -> String {
        let mut response = format!("HTTP/1.1 {} {}\r\n", self.status, self.status_text());
        
        // Add headers
        for (key, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", key, value));
        }
        
        // Add content length header
        response.push_str(&format!("Content-Length: {}\r\n", self.body.len()));
        response.push_str("Connection: close\r\n");
        response.push_str("\r\n");
        response.push_str(&self.body);
        
        response
    }

    fn status_text(&self) -> &str {
        match self.status {
            200 => "OK",
            404 => "Not Found",
            _ => "Unknown"
        }
    }
}

// FIXED: Updated to use proper error handling
async fn parse_request(mut stream: &mut TcpStream) -> Result<HttpRequest, Box<dyn MyError>> {
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).await
        .map_err(|e| Box::new(ServerError::Io(e)) as Box<dyn MyError>)?;
    
    let request_str = String::from_utf8_lossy(&buffer[..bytes_read]);
    let lines: Vec<&str> = request_str.lines().collect();
    
    if lines.is_empty() {
        return Ok(HttpRequest::new("/".to_string(), "GET".to_string()));
    }
    
    let first_line = lines[0];
    let parts: Vec<&str> = first_line.split_whitespace().collect();
    
    if parts.len() >= 2 {
        let method = parts[0].to_string();
        let path = parts[1].to_string();
        Ok(HttpRequest::new(path, method))
    } else {
        Err(Box::new(ServerError::RequestParse(
            "Invalid request format".to_string()
        )) as Box<dyn MyError>)
    }
}

// basic server
pub struct Server {
    addr: SocketAddr,
}

impl Server {
    // FIXED: Updated to use ServerError and proper error conversion
    pub fn new(addr: &str) -> Result<Self, Box<dyn MyError>> {
        let addr = addr.parse()
            .map_err(|e| Box::new(ServerError::AddrParse(e)) as Box<dyn MyError>)?;
        Ok( Self { addr })
    }

    // FIXED: Updated error handling in run method
    pub async fn run<F, Fut>(self, handler: F) -> Result<(), Box<dyn MyError>>
    where 
        F: Fn(HttpRequest) -> HttpResponse + Send + Sync + Clone + 'static,
        Fut: Future<Output = HttpResponse> + Send + 'static,
        {
            let listener = TcpListener::bind(self.addr).await
                .map_err(|e| Box::new(ServerError::Io(e)) as Box<dyn MyError>)?;
            println!("Server running on {}", self.addr);

            loop {
                let (mut connection, _) = listener.accept().await
                    .map_err(|e| Box::new(ServerError::Io(e)) as Box<dyn MyError>)?;
                let handler = handler.clone();

                task::spawn(async move {
                    match parse_request(&mut connection).await {
                        Ok(request) => {
                            println!("Received request: {} {}", request.method(), request.path());
                            
                            let response = handler(request);
                            println!("Response: {:?}", response);
                            
                            let response_str = response.to_http_string();
                            if let Err(e) = connection.write_all(response_str.as_bytes()).await {
                                eprintln!("Failed to send response: {}", e);
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to parse request: {}", e);
                        }
                    }
                });
            }
        }
}

async fn handle_request(request: HttpRequest) -> HttpResponse {
    if request.path() == "/" {
        for i in 0..5 {
            println!("Processing request: {}", i);
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await; // Simulate some processing delay
        }
        HttpResponse::ok("Hello, World!")
    } else if request.path() == "/hello" {
        HttpResponse::ok("Hello, Rust!")
    } else {
        HttpResponse::not_found()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn MyError>>{
    let server = Server::new("127.0.0.1:3000")?;
    server.run(handle_request).await?;
    Ok(())
}