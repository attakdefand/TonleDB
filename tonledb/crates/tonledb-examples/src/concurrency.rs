//! Concurrency examples using Tokio for high-concurrency HTTP server
//!
//! This module demonstrates how to build a high-concurrency HTTP server
//! that can handle thousands of requests on a single-threaded Tokio runtime.

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;

/// A simple counter that can be shared across requests
#[derive(Clone)]
pub struct AppState {
    pub counter: Arc<std::sync::atomic::AtomicUsize>,
}

/// Request payload for incrementing the counter
#[derive(Deserialize)]
pub struct IncrementRequest {
    pub amount: usize,
}

/// Response payload for counter operations
#[derive(Serialize)]
pub struct CounterResponse {
    pub value: usize,
}

/// Health check endpoint
pub async fn health_check() -> impl IntoResponse {
    "OK"
}

/// Get the current counter value
pub async fn get_counter(State(state): State<AppState>) -> impl IntoResponse {
    let value = state.counter.load(std::sync::atomic::Ordering::Relaxed);
    Json(CounterResponse { value })
}

/// Increment the counter by a specified amount
pub async fn increment_counter(
    State(state): State<AppState>,
    Json(payload): Json<IncrementRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let current = state.counter.fetch_add(payload.amount, std::sync::atomic::Ordering::Relaxed);
    Ok(Json(CounterResponse {
        value: current + payload.amount,
    }))
}

/// Run a high-concurrency HTTP server
pub async fn run_high_concurrency_server() -> anyhow::Result<()> {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();

    // Create shared state
    let state = AppState {
        counter: Arc::new(std::sync::atomic::AtomicUsize::new(0)),
    };

    // Build the application with routes
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/counter", get(get_counter))
        .route("/increment", post(increment_counter))
        .with_state(state);

    // Run the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("High-concurrency server listening on {}", addr);
    
    axum::serve(listener, app).await?;
    
    Ok(())
}

/// Example of how to use the high-concurrency server
///
/// ```rust,no_run
/// use tonledb_examples::concurrency;
/// 
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     concurrency::run_high_concurrency_server().await
/// }
/// ```
pub fn example_usage() {
    println!("To run the high-concurrency server, use the run_high_concurrency_server() function");
    println!("The server will be available at http://127.0.0.1:3000");
    println!("Endpoints:");
    println!("  GET  /health     - Health check");
    println!("  GET  /counter    - Get current counter value");
    println!("  POST /increment  - Increment counter (provide {{\"amount\": N}} in body)");
}