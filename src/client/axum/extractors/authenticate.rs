use std::ops::Deref;
use axum::{async_trait, extract::FromRequestParts, http::{request::Parts, StatusCode}};

use crate::client::{AuthClient, VerifyTokenRequest, AUTH_CLIENT};

pub struct ClaimsUser {
    pub id: i32,
}

pub struct Authenticate(pub ClaimsUser);

impl Deref for Authenticate {
    type Target = ClaimsUser;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[async_trait]
impl<T> FromRequestParts<T> for Authenticate {
    type Rejection = StatusCode;

    async fn from_request_parts<'a, 'b>(parts: &'a mut Parts, _: &'b T) -> Result<Self, Self::Rejection> {
        let (scheme, token) = parts.headers
            .get("Authorization")
            .ok_or(StatusCode::UNAUTHORIZED)?
            .to_str()
            .map_err(|_| StatusCode::UNAUTHORIZED)?
            .split_once(' ')
            .ok_or(StatusCode::BAD_REQUEST)?;

        if scheme.to_ascii_uppercase() != "BEARER" {
            return Err(StatusCode::BAD_REQUEST);
        }

        let auth_client = AUTH_CLIENT.get_or_init(AuthClient::default);

        let request = VerifyTokenRequest {
            token: token.to_string(),
        };

        let response = auth_client.verify_token(request)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Authenticate(
            ClaimsUser { 
                id: response.user_id 
            }
        ))
    }
}