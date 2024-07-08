use prost::Message;

#[derive(Message)]
pub struct VerifyTokenResponse {
    #[prost(string, tag = "1")]
    pub user_id: String,
}