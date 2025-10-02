use axum::{response::{IntoResponse, Response}, http::StatusCode, Json};
use serde::Serialize;

#[derive(Serialize)]
struct ApiErr { code: &'static str, message: &'static str }

pub enum AppError {
  BadRequest, Unauthorized, Forbidden, NotFound, Conflict, TooMany, Internal,
}

impl IntoResponse for AppError {
  fn into_response(self) -> Response {
    let (status, body) = match self {
      AppError::BadRequest   => (StatusCode::BAD_REQUEST,    ApiErr{code:"bad_request", message:"invalid request"}),
      AppError::Unauthorized => (StatusCode::UNAUTHORIZED,   ApiErr{code:"unauthorized", message:"auth required"}),
      AppError::Forbidden    => (StatusCode::FORBIDDEN,      ApiErr{code:"forbidden", message:"not allowed"}),
      AppError::NotFound     => (StatusCode::NOT_FOUND,      ApiErr{code:"not_found", message:"resource not found"}),
      AppError::Conflict     => (StatusCode::CONFLICT,       ApiErr{code:"conflict", message:"state conflict"}),
      AppError::TooMany      => (StatusCode::TOO_MANY_REQUESTS, ApiErr{code:"rate_limited", message:"too many requests"}),
      AppError::Internal     => (StatusCode::INTERNAL_SERVER_ERROR, ApiErr{code:"internal", message:"unexpected error"}),
    };
    (status, Json(body)).into_response()
  }
}
