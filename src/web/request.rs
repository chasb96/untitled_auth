use prost::Message;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SignUpRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Message)]
pub struct AuthenticateRequest {
    #[prost(string, tag = "1")]
    pub token: String,
}