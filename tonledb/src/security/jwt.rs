use axum::{http::Request, middleware::Next, response::Response};
use jsonwebtoken::{DecodingKey, Validation, decode, Algorithm, TokenData};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Claims { sub: String, aud: String, scope: Option<String>, exp: usize, iss: String }

pub async fn require_scope<B>(req: Request<B>, next: Next<B>, scope: &str) -> Result<Response, axum::http::StatusCode> {
    let Some(auth) = req.headers().get("authorization").and_then(|v| v.to_str().ok()) else {
        return Err(axum::http::StatusCode::UNAUTHORIZED);
    };
    let token = auth.strip_prefix("Bearer ").ok_or(axum::http::StatusCode::UNAUTHORIZED)?;
    let key = DecodingKey::from_rsa_pem(include_bytes!("../keys/jwks_dev_public.pem")).map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let mut val = Validation::new(Algorithm::RS256);
    val.set_audience(&["tonledb"]);
    let TokenData{claims, ..} = decode::<Claims>(token, &key, &val).map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
    if !claims.scope.unwrap_or_default().split_whitespace().any(|s| s == scope) {
        return Err(axum::http::StatusCode::FORBIDDEN);
    }
    Ok(next.run(req).await)
}
