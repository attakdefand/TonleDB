use tower_http::{
  cors::{Any, CorsLayer},
  set_header::SetResponseHeaderLayer,
};
use http::HeaderValue;

pub fn cors_strict() -> CorsLayer {
    CorsLayer::new()
      .allow_origin("https://your-web-origin.example".parse::<HeaderValue>().unwrap())
      .allow_methods([http::Method::GET, http::Method::POST])
      .allow_headers([http::header::CONTENT_TYPE, http::HeaderName::from_static("authorization"),
                      http::HeaderName::from_static("x-signature"),
                      http::HeaderName::from_static("x-timestamp"),
                      http::HeaderName::from_static("x-nonce")])
      .max_age(std::time::Duration::from_secs(600))
}

pub fn baseline_headers() -> SetResponseHeaderLayer<HeaderValue> {
    SetResponseHeaderLayer::overriding(
      http::HeaderName::from_static("x-content-type-options"),
      HeaderValue::from_static("nosniff"),
    )
}
