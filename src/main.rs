use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AcceptanceFormData {
    services: Services,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Services {
    disinsection: bool,
    deratization: bool,
    disinfection: bool,
    subcontractor_prevention: bool,
}
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
        valid: payload.services.disinsection
            || payload.services.deratization
            || payload.services.disinfection
            || payload.services.subcontractor_prevention,
    })
}
