mod error;
mod request;
mod response;

pub mod axum;

use std::{env, sync::OnceLock};

use prost::Message;
pub use request::VerifyTokenRequest;
pub use response::VerifyTokenResponse;
pub use error::Error;

use reqwest::{header::CONTENT_TYPE, Client};

static AUTH_CLIENT: OnceLock<AuthClient> = OnceLock::new();

pub struct AuthClient {
    http_client: Client,
    base_url: String,
}

impl AuthClient {
    pub fn new(http_client: Client, base_url: String) -> Self {
        Self {
            http_client,
            base_url
        }
    }

    pub async fn verify_token(&self, request: VerifyTokenRequest) -> Result<VerifyTokenResponse, Error> {
        let response = self.http_client
            .post(format!("{}/verify_token", self.base_url))
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
        let base_url = env::var("AUTH_BASE_URL")
            .unwrap_or("http://auth".to_string())
            .trim_end_matches('/')
            .to_string();
        
        Self { 
            http_client: Default::default(),
            base_url,
        }
    }
}