use tokio::{
    self,
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    time::{self, Duration},
};
use std::{error::Error, sync::Arc};
use std::sync::atomic::{AtomicBool, Ordering};

async fn handle_file_transfer_broken(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; 1024];
    let mut file = tokio::fs::File::create("received_file_broken.txt").await?;

    while let Ok(n) = stream.read(&mut buffer).await {
        if n == 0 { break; }
        let _ = file.write(&buffer[..n]).await;
    }
    Ok(())
}

async fn handle_file_transfer_fixed(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; 1024];
    let mut file = tokio::fs::File::create("received_file_fixed.txt").await?;
    let mut total_bytes = 0;

    loop {
        match stream.read(&mut buffer).await {
            Ok(n) if n == 0 => break,
            Ok(n) => {
                file.write_all(&buffer[..n]).await?;
                total_bytes += n;
                file.flush().await?;
            }
            Err(e) => {
                eprintln!("Error reading from stream: {}", e);
                return Err(e.into());
            }
        }
    }

    println!("Successfully received {} bytes", total_bytes);
    Ok(())
}

async fn simulate_unstable_client(port: u16, data: Vec<u8>) -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).await?;
    let chunks = data.chunks(100);
    
    for chunk in chunks {
        if rand::random::<f32>() < 0.3 {
            println!("Simulating network glitch...");
            continue;
        }
        stream.write_all(chunk).await?;
        time::sleep(Duration::from_millis(10)).await;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener_broken = TcpListener::bind("127.0.0.1:8080").await?;
    let listener_fixed = TcpListener::bind("127.0.0.1:8081").await?;
    
    println!("Starting broken server on :8080 and fixed server on :8081");

    let test_data = (0..1000).map(|i| (i % 256) as u8).collect::<Vec<_>>();
    let test_data = Arc::new(test_data);

    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();

    let broken_handle = {
        let test_data = test_data.clone();
        tokio::spawn(async move {
            println!("Broken server listening on :8080");
            loop {
                match time::timeout(Duration::from_secs(1), listener_broken.accept()).await {
                    Ok(Ok((stream, _))) => {
                        if let Err(e) = handle_file_transfer_broken(stream).await {
                            eprintln!("Broken transfer error: {}", e);
                        }
                    }
                    Ok(Err(e)) => eprintln!("Accept error: {}", e),
                    Err(_) => {}
                }
                if !running_clone.load(Ordering::SeqCst) {
                    break;
                }
            }
            Ok::<_, Box<dyn Error + Send>>(())
        })
    };

    let fixed_handle = {
        let test_data = test_data.clone();
        let running_clone = running.clone();
        tokio::spawn(async move {
            println!("Fixed server listening on :8081");
            loop {
                match time::timeout(Duration::from_secs(1), listener_fixed.accept()).await {
                    Ok(Ok((stream, _))) => {
                        if let Err(e) = handle_file_transfer_fixed(stream).await {
                            eprintln!("Fixed transfer error: {}", e);
                        }
                    }
                    Ok(Err(e)) => eprintln!("Accept error: {}", e),
                    Err(_) => {}
                }
                if !running_clone.load(Ordering::SeqCst) {
                    break;
                }
            }
            Ok::<_, Box<dyn Error + Send>>(())
        })
    };

    for _ in 0..5 {
        let data = test_data.clone();
        tokio::spawn(async move {
            if let Err(e) = simulate_unstable_client(8080, data.to_vec()).await {
                eprintln!("Client error (broken): {}", e);
            }
        });

        let data = test_data.clone();
        tokio::spawn(async move {
            if let Err(e) = simulate_unstable_client(8081, data.to_vec()).await {
                eprintln!("Client error (fixed): {}", e);
            }
        });
    }

    tokio::time::sleep(Duration::from_secs(5)).await;
    running.store(false, Ordering::SeqCst);

    let (broken_result, fixed_result) = tokio::join!(broken_handle, fixed_handle);
    broken_result??;
    fixed_result??;

    let broken_file = tokio::fs::read("received_file_broken.txt").await?;
    let fixed_file = tokio::fs::read("received_file_fixed.txt").await?;

    println!("\nResults comparison:");
    println!("Original data size: {}", test_data.len());
    println!("Broken transfer size: {}", broken_file.len());
    println!("Fixed transfer size: {}", fixed_file.len());

    let broken_missing = test_data.len() - broken_file.len();
    let fixed_missing = test_data.len() - fixed_file.len();

    println!("\nMissing bytes:");
    println!("Broken transfer: {} bytes missing", broken_missing);
    println!("Fixed transfer: {} bytes missing", fixed_missing);

    let broken_corrupted = broken_file.iter()
        .zip(test_data.iter())
        .filter(|&(a, b)| a != b)
        .count();
    let fixed_corrupted = fixed_file.iter()
        .zip(test_data.iter())
        .filter(|&(a, b)| a != b)
        .count();

    println!("\nCorrupted bytes:");
    println!("Broken transfer: {} bytes corrupted", broken_corrupted);
    println!("Fixed transfer: {} bytes corrupted", fixed_corrupted);

    Ok(())
}