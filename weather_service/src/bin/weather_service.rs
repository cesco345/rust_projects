use std::time::Duration;
use tokio::time::sleep;
use warp::{Filter, Rejection, Reply};
use serde::{Deserialize, Serialize};
use rand::Rng;

// Weather data structure
#[derive(Debug, Serialize, Deserialize, Clone)]
struct WeatherData {
    temperature: f32,
    humidity: f32,
    description: String,
    fetch_time_ms: u64,
}

// Simulated slow database query (blocking operation)
fn blocking_get_weather() -> WeatherData {
    // Simulate slow database query
    std::thread::sleep(Duration::from_secs(2));
    
    let mut rng = rand::thread_rng();
    WeatherData {
        temperature: rng.gen_range(0.0..35.0),
        humidity: rng.gen_range(30.0..90.0),
        description: "Sunny".to_string(),
        fetch_time_ms: 2000,
    }
}

// Async version of weather fetching
async fn async_get_weather() -> WeatherData {
    // Simulate async database query
    sleep(Duration::from_secs(2)).await;
    
    let mut rng = rand::thread_rng();
    WeatherData {
        temperature: rng.gen_range(0.0..35.0),
        humidity: rng.gen_range(30.0..90.0),
        description: "Sunny".to_string(),
        fetch_time_ms: 2000,
    }
}

// Handler for blocking weather route
async fn get_weather_blocking() -> Result<impl Reply, Rejection> {
    // This will block the thread
    let weather = tokio::task::spawn_blocking(blocking_get_weather).await.unwrap();
    Ok(warp::reply::json(&weather))
}

// Handler for async weather route
async fn get_weather_async() -> Result<impl Reply, Rejection> {
    // This won't block
    let weather = async_get_weather().await;
    Ok(warp::reply::json(&weather))
}

// Handler for multiple async weather stations
async fn get_multiple_weather_async() -> Result<impl Reply, Rejection> {
    // Create multiple concurrent weather fetches
    let fetches = vec![
        async_get_weather(),
        async_get_weather(),
        async_get_weather(),
    ];
    
    // Wait for all to complete
    let results = futures::future::join_all(fetches).await;
    Ok(warp::reply::json(&results))
}

// Serve HTML page
async fn serve_page() -> Result<impl Reply, Rejection> {
    Ok(warp::reply::html(include_str!("../weather.html")))
}

#[tokio::main]
async fn main() {
    println!("Starting weather service on http://localhost:3030");
    println!("Try these endpoints:");
    println!("  /                     - Web interface");
    println!("  /weather/blocking     - Blocking API");
    println!("  /weather/async        - Async API");
    println!("  /weather/multiple     - Multiple async fetches");

    // Define routes
    let routes = warp::path::end()
        .and(warp::get())
        .and_then(serve_page)
        .or(warp::path!("weather" / "blocking")
            .and(warp::get())
            .and_then(get_weather_blocking))
        .or(warp::path!("weather" / "async")
            .and(warp::get())
            .and_then(get_weather_async))
        .or(warp::path!("weather" / "multiple")
            .and(warp::get())
            .and_then(get_multiple_weather_async));

    // Start the server
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}