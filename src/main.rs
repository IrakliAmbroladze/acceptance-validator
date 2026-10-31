use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use models::AcceptanceFormData;
use serde::Serialize;
use tower_http::cors;
use validators::{
    validate_criteria, validate_pests, validate_products, validate_required_fields,
    validate_services, validate_spaces,
};

mod models;
mod validators;

#[derive(Serialize)]
struct ValidateResponse {
    valid: bool,
    error: Option<String>,
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

fn validation_error(message: &str) -> (StatusCode, Json<ValidateResponse>) {
    (
        StatusCode::BAD_REQUEST,
        Json(ValidateResponse {
            valid: false,
            error: Some(message.to_string()),
        }),
    )
}

async fn validate(Json(payload): Json<AcceptanceFormData>) -> impl IntoResponse {
    if !validate_services(&payload.services) {
        return validation_error("მინიმუმ ერთი პროცედურა უნდა იყოს მონიშნული");
    }

    if !validate_pests(&payload.pests) {
        return validation_error("მინიმუმ ერთი გატარებული ღონისძიება უნდა იყოს მონიშნული");
    }

    if !validate_products(&payload.products) {
        return validation_error("მინიმუმ ერთი გახარჯული რაოდენობა უნდა იყოს მითითებული");
    }

    if !validate_spaces(&payload.spaces) {
        return validation_error("მინიმუმ ერთი სივრცე უნდა იყოს მონიშნული");
    }

    if !validate_criteria(&payload.criteria) {
        return validation_error("ინსპექტირებაში ყველა კრიტერიუმი უნდა იყოს მონიშნული");
    }

    if !validate_required_fields(&payload) {
        return validation_error(
            "დამკვეთის სახელი, დაწყების დრო, დასრულების დრო და კომენტარი სავალდებულოა",
        );
    }

    (
        StatusCode::OK,
        Json(ValidateResponse {
            valid: true,
            error: None,
        }),
    )
}
