use ::axum::{routing::get, Router};
use log_layer::LogLayer;
use routes::AuthRouter;

mod jwt;
mod axum;
mod hash;
mod repository;
mod web;
mod configuration;
mod health;
mod routes;


pub fn router() -> Router {
    Router::new()
        .route("/health", get(health::health))
        .register_auth_routes()
        .layer(LogLayer::new())
}