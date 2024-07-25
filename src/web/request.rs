use prost::Message;
use serde::Deserialize;

#[derive(Deserialize, Message)]
pub struct SignUpRequest {
    #[prost(string, tag = "1")]
    pub username: String,
    #[prost(string, tag = "2")]
    pub password: String,
}

#[derive(Deserialize, Message)]
pub struct LoginRequest {
    #[prost(string, tag = "1")]
    pub username: String,
    #[prost(string, tag = "2")]
    pub password: String,
}

#[derive(Deserialize, Message)]
pub struct AuthenticateRequest {
    #[prost(string, tag = "1")]
    pub token: String,
}