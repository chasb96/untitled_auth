use prost::Message;
use serde::Serialize;

#[derive(Serialize)]
pub struct SignUpResponse {
    pub id: i32,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Serialize, Message)] 
pub struct AuthenticateResponse {
    #[prost(int32, tag = "1")]
    pub user_id: i32,
}