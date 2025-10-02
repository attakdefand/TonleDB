use std::{collections::HashMap, fs};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use serde::Deserialize;
use axum::{http::StatusCode, response::{Response, IntoResponse}, extract::FromRequestParts};
use axum::http::request::Parts;

#[derive(Clone, Debug)]
pub struct Identity { pub name: String, pub role: Role }
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Role { Admin, ReadWrite, ReadOnly }
impl Role { pub fn from_str(s: &str) -> Self { match s { "admin"=>Self::Admin, "readwrite"=>Self::ReadWrite, _=>Self::ReadOnly } } }

#[derive(Deserialize)]
struct TokenEntry { name: String, role: String, hash: String }
#[derive(Deserialize)]
struct TokenFile { tokens: Vec<TokenEntry> }

#[derive(Clone)]
pub struct TokenStore { map: HashMap<String,(String,Role)> }
impl TokenStore {
    pub fn from_file(p: &str) -> anyhow::Result<Self> {
        let tf: TokenFile = serde_json::from_str(&fs::read_to_string(p)?)?;
        let mut map = HashMap::new();
        for t in tf.tokens { map.insert(t.name.clone(), (t.hash, Role::from_str(&t.role))); }
        Ok(Self{ map })
    }
    pub fn verify(&self, name:&str, token:&str) -> Option<Identity> {
        let (hash, role) = self.map.get(name)?;
        let parsed = PasswordHash::new(hash).ok()?;
        Argon2::default().verify_password(token.as_bytes(), &parsed).ok()?;
        Some(Identity{ name: name.to_string(), role: role.clone() })
    }
}
impl Default for TokenStore {
    fn default() -> Self {
        Self { map: HashMap::new() }
    }
}
#[derive(Clone)] pub enum AuthMode { None, Token }
#[derive(Clone)] pub struct AppAuth { pub tokens: TokenStore, pub mode: AuthMode }
pub struct User(pub Identity);

#[axum::async_trait]
impl<S> FromRequestParts<S> for User where S: Send+Sync {
    type Rejection = Response;
    async fn from_request_parts(parts:&mut Parts,_:&S)->Result<Self,Self::Rejection>{
        let name = parts.headers.get("x-auth-name").and_then(|v| v.to_str().ok()).unwrap_or("");
        let token= parts.headers.get("x-auth-token").and_then(|v| v.to_str().ok()).unwrap_or("");
        let app = parts.extensions.get::<AppAuth>().ok_or_else(|| (StatusCode::INTERNAL_SERVER_ERROR,"auth missing").into_response())?;
        match app.mode {
            AuthMode::None => Ok(Self(Identity{name:"debug".into(), role:Role::Admin})),
            AuthMode::Token => app.tokens.verify(name, token).map(User).ok_or_else(|| (StatusCode::UNAUTHORIZED,"invalid token").into_response())
        }
    }
}
pub fn require(role: Role, got: &Role)->bool {
    match role {
        Role::Admin => *got==Role::Admin,
        Role::ReadWrite => matches!(got, Role::Admin|Role::ReadWrite),
        Role::ReadOnly => true,
    }
}