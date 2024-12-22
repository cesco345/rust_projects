# Multithreaded Web Server in Rust

This project implements a multithreaded web server using Rust. The server can handle multiple concurrent connections using a thread pool and provides different routes with varying response times to demonstrate concurrent processing.

## Features

- Thread pool implementation for handling concurrent requests
- Multiple routes (/. /sleep, 404)
- Thread-safe statistics tracking
- Detailed logging of server operations
- Timeout handling for connections
- Custom HTML pages for different routes

## Project Structure

```
.
├── src/
│   ├── bin/
│   │   └── multithreaded_server.rs   # Main server implementation
│   ├── lib.rs                        # ThreadPool implementation
│   ├── hello.html                    # Homepage template
│   ├── sleep.html                    # Sleep page template
│   └── 404.html                      # Error page template
└── README.md
```

## Implementation Details

### Key Components

1. **ThreadPool (lib.rs)**

   - Custom implementation of a thread pool
   - Uses channels for job distribution
   - Handles worker thread management
   - Provides clean shutdown capabilities

2. **Server (multithreaded_server.rs)**
   - TCP connection handling
   - HTTP request parsing
   - Route handling
   - Response generation
   - Statistics tracking

### Code Breakdown

#### Imports and Structure

```rust
use hello::ThreadPool;  // Custom thread pool implementation
use std::{
    fs,  // File operations
    io::{prelude::*, BufReader},  // I/O operations
    net::{TcpListener, TcpStream},  // Network operations
    sync::{Arc, Mutex},  // Thread-safe shared state
    thread,  // Thread operations
    time::Duration,  // Time operations
};

// Server statistics structure
struct ServerStats {
    requests: usize,
    active_connections: usize,
}
```

#### Server Setup

```rust
fn main() {
    // Create TCP listener
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // Initialize thread pool
    let pool = ThreadPool::new(4);

    // Create thread-safe statistics
    let stats = Arc::new(Mutex::new(ServerStats {
        requests: 0,
        active_connections: 0,
    }));
```

#### Connection Handling

```rust
for stream in listener.incoming() {
    let stats = Arc::clone(&stats);

    match stream {
        Ok(stream) => {
            // Update statistics
            {
                let mut stats = stats.lock().unwrap();
                stats.requests += 1;
                stats.active_connections += 1;
            }

            // Handle connection in thread pool
            let stats_clone = Arc::clone(&stats);
            pool.execute(move || {
                handle_connection(stream);
                let mut stats = stats_clone.lock().unwrap();
                stats.active_connections -= 1;
            });
        }
        Err(e) => eprintln!("Connection error: {}", e),
    }
}
```

#### Request Processing

```rust
fn handle_connection(mut stream: TcpStream) {
    // Set timeouts
    stream.set_read_timeout(Some(Duration::from_secs(5))).unwrap();

    // Read request
    let buf_reader = BufReader::new(&stream);
    let request_line = match buf_reader.lines().next() {
        Some(Ok(line)) => line,
        Some(Err(e)) => {
            eprintln!("Error reading request line: {}", e);
            return;
        }
        None => {
            eprintln!("Empty request");
            return;
        }
    };
```

#### Route Handling

```rust
let (status_line, filename, sleep_duration) = match &request_line[..] {
    "GET / HTTP/1.1" => {
        ("HTTP/1.1 200 OK", "src/hello.html", None)
    }
    "GET /sleep HTTP/1.1" => {
        ("HTTP/1.1 200 OK", "src/sleep.html", Some(Duration::from_secs(5)))
    }
    _ => {
        ("HTTP/1.1 404 NOT FOUND", "src/404.html", None)
    }
};
```

## Usage

1. Start the server:

```bash
cargo run --bin multithreaded_server
```

2. Visit these URLs in your browser:

- http://localhost:7878/ - Home page
- http://localhost:7878/sleep - Page with 5-second delay
- http://localhost:7878/anything-else - 404 error page

3. Test concurrent processing:

```bash
curl http://localhost:7878/ & curl http://localhost:7878/sleep & curl http://localhost:7878/
```

## Key Concepts

1. **Thread Pool Pattern**

   - Fixed number of worker threads
   - Job queue using channels
   - Worker thread management

2. **Shared State**

   - Arc for thread-safe reference counting
   - Mutex for safe data access
   - Statistics tracking

3. **Connection Handling**

   - Non-blocking I/O
   - Timeout implementation
   - HTTP parsing

4. **Concurrency Features**

   - Simultaneous request handling
   - Thread-safe state management
   - Request timing

5. **Error Handling**
   - Network error handling
   - File system error handling
   - Timeout management
   - Resource cleanup

## Contributing

Feel free to submit issues and enhancement requests!

## License

[MIT](LICENSE)
