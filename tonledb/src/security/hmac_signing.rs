use axum::{http::{Request, HeaderMap, StatusCode}, middleware::Next, response::Response};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use base64::{engine::general_purpose::STANDARD as B64, Engine};
use time::{OffsetDateTime, Duration};
use redis::{AsyncCommands, Client as RedisClient};
use http::header::HeaderName;
use anyhow::Result;

type HmacSha256 = Hmac<Sha256>;

const HDR_SIG: &str = "x-signature";
const HDR_TS:  &str = "x-timestamp";
const HDR_NONCE:&str = "x-nonce";
const WINDOW_SECS: i64 = 120;

fn derive_signing_string(method: &str, path: &str, ts: &str, nonce: &str, body_sha256_b64: &str) -> String {
    format!("{}|{}|{}|{}|{}", method, path, ts, nonce, body_sha256_b64)
}

async fn body_hash_b64<B>(req: &Request<B>) -> Result<String>
where B: http_body::Body + Send, <B as http_body::Body>::Error: std::fmt::Display {
    // Use content-length=0 shortcut; otherwise buffer via axum::body::to_bytes in a real impl.
    Ok(B64.encode(Sha256::digest(b"").as_slice()))
}

pub async fn verify_hmac<B>(
    mut req: Request<B>,
    next: Next<B>,
    secret: &'static [u8],
    redis: RedisClient,
) -> Result<Response, StatusCode> {
    let headers = req.headers();
    let sig  = headers.get(HeaderName::from_static(HDR_SIG)).and_then(|v| v.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;
    let ts   = headers.get(HeaderName::from_static(HDR_TS)).and_then(|v| v.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;
    let nonce= headers.get(HeaderName::from_static(HDR_NONCE)).and_then(|v| v.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Replay window
    let ts_i = ts.parse::<i64>().map_err(|_| StatusCode::UNAUTHORIZED)?;
    let now  = OffsetDateTime::now_utc().unix_timestamp();
    if (now - ts_i).abs() > WINDOW_SECS { return Err(StatusCode::UNAUTHORIZED); }

    // Nonce check (SETNX with expiry ~ window)
    let mut r = redis.get_async_connection().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let key = format!("nonce:{}", nonce);
    let set: bool = r.set_nx(&key, "1").await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if !set { return Err(StatusCode::UNAUTHORIZED); }
    let _: () = r.expire(&key, (WINDOW_SECS * 2) as usize).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Compute signing string
    let method = req.method().as_str();
    let path   = req.uri().path_and_query().map(|pq| pq.as_str()).unwrap_or(req.uri().path());
    let body_b64 = body_hash_b64(&req).await.map_err(|_| StatusCode::BAD_REQUEST)?;
    let msg = derive_signing_string(method, path, ts, nonce, &body_b64);

    // Verify HMAC
    let mut mac = HmacSha256::new_from_slice(secret).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    mac.update(msg.as_bytes());
    let expected = B64.encode(mac.finalize().into_bytes());
    if subtle::ConstantTimeEq::ct_eq(expected.as_bytes(), sig.as_bytes()).unwrap_u8() != 1 {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(req).await)
}
