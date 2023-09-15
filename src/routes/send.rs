use crate::services;
use axum::{http::StatusCode, Json};
use serde::Deserialize;
use tokio;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct Payload {
    #[validate(url)]
    pub url: String,
    #[validate(email)]
    pub email: String,
}

pub async fn send_handler(Json(payload): Json<Payload>) -> (StatusCode, String) {
    match payload.validate() {
        Ok(_) => {
            // tokio::spawn(async move {
            let pdf = match services::pdf::gen_pdf(&payload.url).await {
                Ok(v) => v,
                Err(err) => {
                    eprintln!("{}", err);
                    Vec::new()
                }
            };
            let _res = services::email::send_email(&pdf, &payload.email, &payload.url).await;
            // });

            (StatusCode::OK, "pdf sent".to_string())
        }
        Err(e) => match e.field_errors().keys().next() {
            Some(x) => (StatusCode::BAD_REQUEST, "invalid ".to_string() + x),
            None => (StatusCode::BAD_REQUEST, "error".to_string()),
        },
    }
}
