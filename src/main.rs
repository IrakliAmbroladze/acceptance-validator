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

async fn validate(Json(payload): Json<AcceptanceFormData>) -> Json<ValidateResponse> {
    //println!("{:#?}", payload);
    if !validate_services(&payload.services) {
        return Json(ValidateResponse {
            valid: false,
            error: Some("მინიმუმ ერთი პროცედურა უნდა იყოს მონიშნული".into()),
        });
    }
    if !validate_pests(&payload.pests) {
        return Json(ValidateResponse {
            valid: false,
            error: Some("მინიმუმ ერთი გატარებული ღონისძიება უნდა იყოს მონიშნული".into()),
        });
    }
    if !validate_products(&payload.products) {
        return Json(ValidateResponse {
            valid: false,
            error: Some("მინიმუმ ერთი გახარჯული რაოდენობა უნდა იყოს მითითებული".into()),
        });
    }
    if !validate_spaces(&payload.spaces) {
        return Json(ValidateResponse {
            valid: false,
            error: Some("მინიმუმ ერთი სივრცე უნდა იყოს მონიშნული".into()),
        });
    }
    if !validate_criteria(&payload.criteria) {
        return Json(ValidateResponse {
            valid: false,
            error: Some("ინსპექტირებაში ყველა კრიტერიუმი უნდა იყოს მონიშნული".into()),
        });
    }
    if !validate_required_fields(&payload) {
        return Json(ValidateResponse {
            valid: false,
            error: Some(
                "დამკვეთის სახელი, დაწყების დრო, დასრულების დრო და კომენტარი სავალდებულოა".into(),
            ),
        });
    }
    Json(ValidateResponse {
        valid: true,
        error: None,
    })
}
