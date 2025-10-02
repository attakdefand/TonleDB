use axum::{Router, routing::get, middleware};
use redis::Client as RedisClient;
use std::net::SocketAddr;

mod security;
use security::hmac_signing::verify_hmac;

static HMAC_SECRET: &[u8] = b"REPLACE_ME_WITH_32B_SECRET";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let redis = RedisClient::open("redis://127.0.0.1/")?;
    let middleware_layer = middleware::from_fn(move |req, next| {
        let redis = redis.clone();
        verify_hmac(req, next, HMAC_SECRET, redis)
    });

    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .layer(middleware_layer);

    let addr: SocketAddr = "0.0.0.0:8080".parse()?;
    println!("API listening on {addr}");
    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;
    Ok(())
}
