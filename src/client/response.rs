use prost::Message;

#[derive(Message)]
pub struct VerifyTokenResponse {
    #[prost(int32, tag = "1")]
    pub user_id: i32,
}