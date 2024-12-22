use hello::ThreadPool;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

struct ServerStats {
    requests: usize,
    active_connections: usize,
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    let stats = Arc::new(Mutex::new(ServerStats {
        requests: 0,
        active_connections: 0,
    }));

    println!("Server listening on http://127.0.0.1:7878");
    println!("Try visiting these urls in different tabs:");
    println!("- http://127.0.0.1:7878/       (home page)");
    println!("- http://127.0.0.1:7878/sleep  (5-second delay page)");

    for stream in listener.incoming() {
        let stats = Arc::clone(&stats);

        match stream {
            Ok(stream) => {
                // Update statistics
                {
                    let mut stats = stats.lock().unwrap();
                    stats.requests += 1;
                    stats.active_connections += 1;
                    println!(
                        "\nActive connections: {}, Total requests: {}",
                        stats.active_connections, stats.requests
                    );
                }

                let stats_clone = Arc::clone(&stats);
                pool.execute(move || {
                    handle_connection(stream);

                    // Update active connections count when done
                    let mut stats = stats_clone.lock().unwrap();
                    stats.active_connections -= 1;
                });
            }
            Err(e) => eprintln!("Connection error: {}", e),
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .unwrap();

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

    let thread_id = thread::current().id();
    println!("Thread {:?} handling request: {}", thread_id, request_line);

    let (status_line, filename, sleep_duration) = match &request_line[..] {
        "GET / HTTP/1.1" => {
            println!("Thread {:?}: Serving index page", thread_id);
            ("HTTP/1.1 200 OK", "src/hello.html", None)
        }
        "GET /sleep HTTP/1.1" => {
            println!(
                "Thread {:?}: Serving sleep page (will take 5 seconds)",
                thread_id
            );
            (
                "HTTP/1.1 200 OK",
                "src/sleep.html",
                Some(Duration::from_secs(5)),
            )
        }
        _ => {
            println!("Thread {:?}: Serving 404 page", thread_id);
            ("HTTP/1.1 404 NOT FOUND", "src/404.html", None)
        }
    };

    if let Some(duration) = sleep_duration {
        println!(
            "Thread {:?}: Sleeping for {} seconds...",
            thread_id,
            duration.as_secs()
        );
        thread::sleep(duration);
        println!("Thread {:?}: Finished sleeping", thread_id);
    }

    let contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Error reading {}: {}", filename, e);
            String::from("Internal Server Error")
        }
    };

    let length = contents.len();
    let response = format!(
        "{}\r\nContent-Length: {}\r\nContent-Type: text/html; charset=utf-8\r\nX-Powered-By: Rust\r\nX-Thread-ID: {:?}\r\n\r\n{}",
        status_line,
        length,
        thread_id,
        contents
    );

    stream
        .set_write_timeout(Some(Duration::from_secs(5)))
        .unwrap();

    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("Thread {:?}: Error writing response: {}", thread_id, e);
    } else {
        println!("Thread {:?}: Response sent successfully", thread_id);
    }
}
