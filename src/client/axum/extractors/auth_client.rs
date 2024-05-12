use std::{ops::Deref, sync::OnceLock};
use axum::{async_trait, extract::FromRequestParts, http::{request::Parts, StatusCode}};

use crate::client::AuthClient as ClientInner;

static AUTH_CLIENT: OnceLock<ClientInner> = OnceLock::new();

pub struct AuthClient(pub &'static ClientInner);

impl Deref for AuthClient {
    type Target = ClientInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[async_trait]
impl<T> FromRequestParts<T> for AuthClient {
    type Rejection = StatusCode;

    async fn from_request_parts<'a, 'b>(_: &'a mut Parts, _: &'b T) -> Result<Self, Self::Rejection> {
        let client = AUTH_CLIENT.get_or_init(ClientInner::default);

        Ok(AuthClient(client))
    }
}