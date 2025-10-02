use axum::{http::{HeaderMap, HeaderValue}, response::IntoResponse};

pub fn etag_for(version: &str) -> HeaderMap {
  let mut h = HeaderMap::new();
  h.insert("ETag", HeaderValue::from_str(&format!("W/\"{version}\"")).unwrap());
  h
}
