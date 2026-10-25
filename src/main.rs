use axum::{
    routing::{get, post},
    Json, Router,
};
use models::AcceptanceFormData;
use serde::Serialize;
use validators::validate_services;

mod models;
mod validators;

#[derive(Serialize)]
struct ValidateResponse {
    valid: bool,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health))
        .route("/validate", post(validate));
    let addr = "0.0.0.0:8080";

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Listening on {addr}");

    axum::serve(listener, app).await.unwrap();
}

async fn health() -> &'static str {
    "ok"
}

async fn validate(Json(payload): Json<AcceptanceFormData>) -> Json<ValidateResponse> {
    Json(ValidateResponse {
        valid: validate_services(&payload.services),
    })
}
