use prost::Message;

#[derive(Message)]
pub struct VerifyTokenRequest {
    #[prost(string, tag = "1")]
    pub token: String,
}