use std::ops::Deref;
use axum::{async_trait, extract::FromRequestParts, http::{request::Parts, StatusCode}};

use crate::client::{self, AuthClient, VerifyTokenRequest, AUTH_CLIENT};

pub struct ClaimsUser {
    pub id: i32,
}

pub struct Authenticate<T>(pub T);

impl<T> Deref for Authenticate<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(PartialEq)]
enum Scheme {
    Unknown,
    Bearer,
}

impl From<&str> for Scheme {
    fn from(scheme: &str) -> Self {
        match scheme.to_ascii_uppercase().as_str() {
            "BEARER" => Self::Bearer,
            _ => Self::Unknown,
        }
    }
}

#[async_trait]
impl<T> FromRequestParts<T> for Authenticate<ClaimsUser> {
    type Rejection = StatusCode;

    async fn from_request_parts<'a, 'b>(parts: &'a mut Parts, _: &'b T) -> Result<Self, Self::Rejection> {
        let (scheme, token) = parts.headers
            .get("Authorization")
            .and_then(|header| header.to_str().ok())
            .and_then(|header| header.split_once(' '))
            .map(|(scheme, token)| (Scheme::from(scheme), token))
            .ok_or(StatusCode::UNAUTHORIZED)?;

        if scheme != Scheme::Bearer {
            return Err(StatusCode::BAD_REQUEST);
        }

        let auth_client = AUTH_CLIENT.get_or_init(AuthClient::default);

        let request = VerifyTokenRequest {
            token: token.to_string(),
        };

        match auth_client.verify_token(request).await {
            Ok(response) => Ok(Self(
                ClaimsUser { 
                    id: response.user_id 
                }
            )),
            Err(client::Error::Status(status)) => Err(status),
            _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

#[async_trait]
impl<T> FromRequestParts<T> for Authenticate<Option<ClaimsUser>> {
    type Rejection = StatusCode;

    async fn from_request_parts<'a, 'b>(parts: &'a mut Parts, _: &'b T) -> Result<Self, Self::Rejection> {
        let authentication = parts.headers
            .get("Authorization")
            .and_then(|header| header.to_str().ok())
            .and_then(|header| header.split_once(' '))
            .map(|(scheme, token)| (Scheme::from(scheme), token));

        let token = match authentication {
            Some((Scheme::Bearer, token)) => token,
            _ => return Ok(Self(None)),
        };

        let auth_client = AUTH_CLIENT.get_or_init(AuthClient::default);

        let request = VerifyTokenRequest {
            token: token.to_string(),
        };

        match auth_client.verify_token(request).await {
            Ok(response) => Ok(Self(Some(
                ClaimsUser { 
                    id: response.user_id 
                }
            ))),
            Err(client::Error::Status(status)) => Err(status),
            _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}