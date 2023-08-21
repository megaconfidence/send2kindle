use axum::{
    Json,
    http::StatusCode,
};
use crate::services;
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct Payload {
    pub url: String,
    pub email: String,
}

pub async fn send_handler(Json(payload): Json<Payload>) -> (StatusCode, String) {
    let pdf = services::pdf::gen_pdf(&payload.url).await;
    let _res =services::email::send_email(&pdf.unwrap(), &payload.email, &payload.url).await;  

    (StatusCode::OK, "pdf sent".to_string())
}
