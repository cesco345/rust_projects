use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    // Create a TCP listener bound to localhost:7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // Handle incoming connections
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // Create a buffered reader for the TCP stream
    let buf_reader = BufReader::new(&stream);

    // Read and collect HTTP request headers
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:?}", http_request);

    // Prepare the HTTP response
    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("src/hello.html").unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    // Send the response
    stream.write_all(response.as_bytes()).unwrap();
}
