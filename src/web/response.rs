use prost::Message;
use serde::Serialize;

#[derive(Serialize, Message)]
pub struct SignUpResponse {
    #[prost(string, tag = "1")]
    pub id: String,
}

#[derive(Serialize, Message)]
pub struct LoginResponse {
    #[prost(string, tag = "1")]
    pub token: String,
}

#[derive(Serialize, Message)] 
pub struct AuthenticateResponse {
    #[prost(string, tag = "1")]
    pub user_id: String,
}