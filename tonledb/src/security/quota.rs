use axum::{http::Request, middleware::Next, response::Response};
use redis::{AsyncCommands, Client};
use std::time::{Duration, SystemTime};
use once_cell::sync::Lazy;

static WINDOW: i64 = 1;       // seconds
static CAPACITY: i64 = 10;    // tokens per window

pub static REDIS: Lazy<Client> = Lazy::new(|| Client::open("redis://127.0.0.1/").unwrap());

pub async fn per_key_quota<B>(req: Request<B>, next: Next<B>) -> Result<Response, axum::http::StatusCode> {
  let key = req.headers().get("x-api-key").and_then(|v| v.to_str().ok()).ok_or(axum::http::StatusCode::UNAUTHORIZED)?;
  let mut r = REDIS.get_async_connection().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
  let bucket = format!("ratelimit:{key}");
  let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64;
  let window_key = format!("{bucket}:{}", now / WINDOW);
  let cnt: i64 = r.incr(&window_key, 1).await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
  let _: () = r.expire(&window_key, (WINDOW*2) as usize).await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
  if cnt > CAPACITY { return Err(axum::http::StatusCode::TOO_MANY_REQUESTS); }
  Ok(next.run(req).await)
}
