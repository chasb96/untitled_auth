use axum::http::StatusCode;
use axum_extra::protobuf::Protobuf;
use or_status_code::{OrInternalServerError, OrStatusCode};
use users_client::axum::extractors::UsersClient;
use users_client::CreateUserRequest;
use users_client::Error as UserClientError;

use crate::axum::extractors::user_repository::UserRepositoryExtractor;
use crate::hash::scrypt::{generate_password_hash, verify_password};
use crate::repository::users::UserRepository;
use crate::jwt::{generate_jwt, verify_jwt, ClaimsUser};

use super::request::{AuthenticateRequest, LoginRequest, SignUpRequest};
use super::response::{AuthenticateResponse, LoginResponse, SignUpResponse};

pub async fn sign_up(
    users_client: UsersClient,
    user_repository: UserRepositoryExtractor,
    Protobuf(request): Protobuf<SignUpRequest>
) -> Result<Protobuf<SignUpResponse>, StatusCode> {
    let create_user_response = users_client
        .create_user(CreateUserRequest { 
            username: request.username 
        })
        .await;

    let create_user_response = match create_user_response {
        Ok(create_user_response) => create_user_response,
        Err(UserClientError::Status(status_code)) => return Err(status_code),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let password_hash = generate_password_hash(&request.password)
        .or_internal_server_error()?;

    user_repository
        .set_password(&create_user_response.id, &password_hash)
        .await
        .or_internal_server_error()?;

    let response = SignUpResponse {
        id: create_user_response.id,
    };

    Ok(Protobuf(response))
}

pub async fn login(
    user_repository: UserRepositoryExtractor,
    Protobuf(request): Protobuf<LoginRequest>
) -> Result<Protobuf<LoginResponse>, StatusCode> {
    let user = user_repository
        .get_by_username(&request.username)
        .await
        .or_internal_server_error()?
        .or_status_code(StatusCode::UNAUTHORIZED)?;

    if !verify_password(&request.password, &user.password_hash).or_internal_server_error()? {
        return Err(StatusCode::UNAUTHORIZED)
    }

    let response = LoginResponse {
        token: generate_jwt(ClaimsUser::from(user))
            .or_internal_server_error()?,
    };

    Ok(Protobuf(response))
}

pub async fn verify_token(
    Protobuf(request): Protobuf<AuthenticateRequest>
) -> Result<Protobuf<AuthenticateResponse>, StatusCode> {
    let claims_user = match verify_jwt::<ClaimsUser>(request.token.clone()) {
        Ok(claims_user) => claims_user,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    let response = AuthenticateResponse {
        user_id: claims_user.id
    };

    Ok(Protobuf(response))
}