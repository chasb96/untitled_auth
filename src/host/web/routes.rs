use axum::{http::StatusCode, Json};
use log::error;

use crate::host::axum::extractors::user_repository::UserRepositoryExtractor;
use crate::host::hash::scrypt::{generate_password_hash, verify_password};
use crate::host::repository::users::error::SignUpError;
use crate::host::repository::users::UserRepository;
use crate::host::axum::JsonOrProtobuf;
use crate::host::jwt::{generate_jwt, verify_jwt, ClaimsUser};
use crate::host::util::or_status_code::{OrInternalServerError, OrStatusCode};

use super::request::{AuthenticateRequest, LoginRequest, SignUpRequest};
use super::response::{AuthenticateResponse, LoginResponse, SignUpResponse};

pub async fn sign_up(
    user_repository: UserRepositoryExtractor,
    Json(request): Json<SignUpRequest>
) -> Result<Json<SignUpResponse>, StatusCode> {
    let password_hash = generate_password_hash(&request.password)
        .or_internal_server_error()?;

    let id = user_repository
        .create_user(&request.username, &password_hash)
        .await
        .map_err(|err| match err {
            SignUpError::UsernameTaken => StatusCode::BAD_REQUEST,
            e => {
                error!("{:?}", e);
                
                StatusCode::INTERNAL_SERVER_ERROR
            },
        })?;

    Ok(Json(
        SignUpResponse {
            id,
        }
    ))
}

pub async fn login(
    user_repository: UserRepositoryExtractor,
    Json(request): Json<LoginRequest>
) -> Result<Json<LoginResponse>, StatusCode> {
    let user = user_repository
        .get_by_username(&request.username)
        .await
        .or_internal_server_error()?
        .or_status_code(StatusCode::UNAUTHORIZED)?;

    if !verify_password(&request.password, &user.password_hash).or_internal_server_error()? {
        return Err(StatusCode::UNAUTHORIZED)
    }

    Ok(Json(
        LoginResponse {
            token: generate_jwt(ClaimsUser::from(user))
                .or_internal_server_error()?,
        }
    ))
}

pub async fn verify_token(
    request: JsonOrProtobuf<AuthenticateRequest>
) -> Result<JsonOrProtobuf<AuthenticateResponse>, StatusCode> {
    let (body, content_type) = request.decompose();

    let claims_user: ClaimsUser = match verify_jwt(body.token) {
        Ok(claims_user) => claims_user,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    let response = AuthenticateResponse {
        user_id: claims_user.id
    };

    Ok(JsonOrProtobuf::new(response, &content_type))
}