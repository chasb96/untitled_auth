use prost::Message;
use serde::Serialize;

#[derive(Serialize)]
pub struct SignUpResponse {
    pub id: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Serialize, Message)] 
pub struct AuthenticateResponse {
    #[prost(string, tag = "1")]
    pub user_id: String,
}