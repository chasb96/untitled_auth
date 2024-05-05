mod error;
mod request;
mod response;

use prost::Message;
pub use request::VerifyTokenRequest;
pub use response::VerifyTokenResponse;
pub use error::Error;

use reqwest::{header::CONTENT_TYPE, Client};

pub struct AuthClient {
    http_client: Client,
}

impl AuthClient {
    pub fn new(http_client: Client) -> Self {
        Self {
            http_client
        }
    }

    pub async fn verify_token(&self, request: VerifyTokenRequest) -> Result<VerifyTokenResponse, Error> {
        let response = self.http_client
            .post("http://auth/verify_token")
            .header(CONTENT_TYPE, "application/octet-stream")
            .body(request.encode_to_vec())
            .send()
            .await?;

        let response_bytes = response.bytes().await?;

        let response = VerifyTokenResponse::decode(response_bytes)?;

        Ok(response)
    }
}

impl Default for AuthClient {
    fn default() -> Self {
        Self { 
            http_client: Default::default() 
        }
    }
}