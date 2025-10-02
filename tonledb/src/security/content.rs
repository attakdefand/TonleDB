use tower_http::validate_request::ValidateRequestHeaderLayer;
use http::header::CONTENT_TYPE;

pub fn json_only() -> ValidateRequestHeaderLayer {
  ValidateRequestHeaderLayer::accept_header(CONTENT_TYPE, "application/json")
}
