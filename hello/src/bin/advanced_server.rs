use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    // Create a TCP listener bound to localhost:7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server listening on http://127.0.0.1:7878");

    // Handle incoming connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    // Create a buffered reader for the TCP stream
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

    // Log the request
    println!("Received request: {}", request_line);

    // Match different routes
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => {
            println!("Serving index page");
            ("HTTP/1.1 200 OK", "src/hello.html")
        }
        "GET /sleep HTTP/1.1" => {
            println!("Serving sleep route");
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "src/hello.html")
        }
        _ => {
            println!("Serving 404 page");
            ("HTTP/1.1 404 NOT FOUND", "src/404.html")
        }
    };

    // Read the file contents with error handling
    let contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Error reading file {}: {}", filename, e);
            String::from("Internal Server Error")
        }
    };

    let length = contents.len();

    // Create and send the response
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    // Send response with error handling
    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("Error sending response: {}", e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::thread;

    #[test]
    fn test_server_response() {
        // Start server in a separate thread
        thread::spawn(|| {
            main();
        });

        // Give the server time to start
        thread::sleep(Duration::from_millis(100));

        // Test basic GET request
        if let Ok(mut stream) = TcpStream::connect("127.0.0.1:7878") {
            stream.write_all(b"GET / HTTP/1.1\r\n\r\n").unwrap();

            // Read response
            let mut buffer = [0; 1024];
            if let Ok(_) = stream.read(&mut buffer) {
                let response = String::from_utf8_lossy(&buffer);
                assert!(response.contains("HTTP/1.1 200 OK"));
                assert!(response.contains("Hello!"));
            }
        }
    }
}
