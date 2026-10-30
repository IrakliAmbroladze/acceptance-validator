use axum::{
    routing::{get, post},
    Json, Router,
};
use models::AcceptanceFormData;
use serde::Serialize;
use tower_http::cors;
use validators::{validate_pests, validate_products, validate_services, validate_spaces};

use crate::validators::{validate_criteria, validate_required_fields};

mod models;
mod validators;

#[derive(Serialize)]
struct ValidateResponse {
    valid: bool,
}

#[tokio::main]
async fn main() {
    let cors = cors::CorsLayer::permissive();
    let app = Router::new()
        .route("/health", get(health))
        .route("/validate", post(validate))
        .layer(cors);
    let addr = "0.0.0.0:8080";

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Listening on {addr}");

    axum::serve(listener, app).await.unwrap();
}

async fn health() -> &'static str {
    "ok"
}

async fn validate(Json(payload): Json<AcceptanceFormData>) -> Json<ValidateResponse> {
    println!("{:#?}", payload);
    Json(ValidateResponse {
        valid: validate_services(&payload.services)
            && validate_pests(&payload.pests)
            && validate_products(&payload.products)
            && validate_spaces(&payload.spaces)
            && validate_criteria(&payload.criteria)
            && validate_required_fields(&payload),
    })
}
