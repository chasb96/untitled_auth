use axum::{http::StatusCode, Json};
use json_or_protobuf::JsonOrProtobuf;
use or_status_code::{OrInternalServerError, OrStatusCode};
use users::client::axum::extractors::UsersClient;
use users::client::{self, CreateUserRequest};

use crate::host::axum::extractors::user_repository::UserRepositoryExtractor;
use crate::host::hash::scrypt::{generate_password_hash, verify_password};
use crate::host::repository::users::UserRepository;
use crate::host::jwt::{generate_jwt, verify_jwt, ClaimsUser};

use super::request::{AuthenticateRequest, LoginRequest, SignUpRequest};
use super::response::{AuthenticateResponse, LoginResponse, SignUpResponse};

pub async fn sign_up(
    users_client: UsersClient,
    user_repository: UserRepositoryExtractor,
    Json(request): Json<SignUpRequest>
) -> Result<Json<SignUpResponse>, StatusCode> {
    let create_user_response = users_client
        .create_user(CreateUserRequest { 
            username: request.username 
        })
        .await;

    let create_user_response = match create_user_response {
        Ok(create_user_response) => create_user_response,
        Err(client::Error::Status(status_code)) => return Err(status_code),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let password_hash = generate_password_hash(&request.password)
        .or_internal_server_error()?;

    user_repository
        .set_password(&create_user_response.id, &password_hash)
        .await
        .or_internal_server_error()?;

    Ok(Json(
        SignUpResponse {
            id: create_user_response.id,
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

    let claims_user = match verify_jwt::<ClaimsUser>(body.token.clone()) {
        Ok(claims_user) => claims_user,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    let response = AuthenticateResponse {
        user_id: claims_user.id
    };

    Ok(JsonOrProtobuf::new(response, &content_type).unwrap())
}