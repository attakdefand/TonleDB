use axum::{extract::Query, http::StatusCode};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Page { pub limit: Option<u32>, pub cursor: Option<String> }

pub fn clamp_limit(p: &Page) -> Result<u32,(StatusCode,String)> {
  let n = p.limit.unwrap_or(50);
  if n == 0 || n > 200 { return Err((StatusCode::BAD_REQUEST,"limit must be 1..=200".into())); }
  Ok(n)
}
