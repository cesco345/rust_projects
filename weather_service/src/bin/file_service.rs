use warp::{Filter, Rejection, Reply};
use tokio::sync::broadcast;
use serde::{Deserialize, Serialize};
use std::{sync::Arc, time::Duration};
use tokio::time::sleep;
use futures_util::{SinkExt, StreamExt};
use tokio_stream::wrappers::BroadcastStream;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TransferStatus {
    bytes_total: usize,
    bytes_received: usize,
    corrupted_chunks: usize,
    missing_chunks: usize,
    is_broken_transfer: bool,
}

async fn simulate_transfer(is_broken: bool) -> TransferStatus {
    let total_chunks = 100;
    let chunk_size = 1024;
    let bytes_total = total_chunks * chunk_size;
    let mut bytes_received = 0;
    let mut corrupted_chunks = 0;
    let mut missing_chunks = 0;
    let mut rng = rand::thread_rng();

    for _ in 0..total_chunks {
        sleep(Duration::from_millis(50)).await;
        
        if is_broken {
            let random = rng.gen::<f32>();
            if random < 0.1 {
                missing_chunks += 1;
                continue;
            }
            if random < 0.2 {
                corrupted_chunks += 1;
            }
            bytes_received += chunk_size;
        } else {
            bytes_received += chunk_size;
        }
    }

    TransferStatus {
        bytes_total,
        bytes_received,
        corrupted_chunks,
        missing_chunks,
        is_broken_transfer: is_broken,
    }
}

async fn handle_broken_transfer(
    status_tx: Arc<broadcast::Sender<TransferStatus>>,
) -> Result<impl Reply, Rejection> {
    let status = simulate_transfer(true).await;
    let _ = status_tx.send(status.clone());
    Ok(warp::reply::json(&status))
}

async fn handle_fixed_transfer(
    status_tx: Arc<broadcast::Sender<TransferStatus>>,
) -> Result<impl Reply, Rejection> {
    let status = simulate_transfer(false).await;
    let _ = status_tx.send(status.clone());
    Ok(warp::reply::json(&status))
}

async fn handle_ws_upgrade(
    ws: warp::ws::Ws,
    mut status_rx: broadcast::Receiver<TransferStatus>,
) -> Result<impl Reply, Rejection> {
    Ok(ws.on_upgrade(move |websocket| handle_ws_connection(websocket, status_rx)))
}

async fn handle_ws_connection(
    websocket: warp::ws::WebSocket,
    status_rx: broadcast::Receiver<TransferStatus>,
) {
    let (mut ws_tx, _) = websocket.split();
    let mut status_stream = BroadcastStream::new(status_rx);

    while let Some(Ok(status)) = status_stream.next().await {
        let msg = warp::ws::Message::text(serde_json::to_string(&status).unwrap());
        if ws_tx.send(msg).await.is_err() {
            break;
        }
    }
}

#[tokio::main]
async fn main() {
    let (status_tx, _) = broadcast::channel(100);
    let status_tx = Arc::new(status_tx);

    let status_tx_filter = warp::any().map(move || Arc::clone(&status_tx));
    
    let broken_transfer = warp::path!("transfer" / "broken")
        .and(warp::get())
        .and(status_tx_filter.clone())
        .and_then(handle_broken_transfer);

    let fixed_transfer = warp::path!("transfer" / "fixed")
        .and(warp::get())
        .and(status_tx_filter.clone())
        .and_then(handle_fixed_transfer);

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(warp::any().map(move || status_tx.subscribe()))
        .and_then(handle_ws_upgrade);

    let content = warp::path::end()
        .and(warp::get())
        .and(warp::fs::file("src/transfer.html"));

    let routes = content
        .or(broken_transfer)
        .or(fixed_transfer)
        .or(ws_route)
        .with(warp::cors().allow_any_origin());

    println!("File transfer demo server starting on http://localhost:3031");
    warp::serve(routes).run(([127, 0, 0, 1], 3031)).await;
}