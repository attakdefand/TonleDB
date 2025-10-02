use axum::{extract::Json, http::StatusCode};
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateItem {
  #[validate(length(min=1, max=64))]
  key: String,
  #[validate(length(min=0, max=1024))]
  value: String,
}

pub async fn create(Json(payload): Json<CreateItem>) -> Result<&'static str, (StatusCode, String)> {
  payload.validate().map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
  // TODO: enforce pagination/scan caps at handler/service level too
  Ok("ok")
}
