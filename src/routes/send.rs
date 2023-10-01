use crate::services;
use axum::{http::StatusCode, Json};
use axum_client_ip::InsecureClientIp;
use serde::{Deserialize, Serialize};
use tokio;
use validator::Validate;

#[derive(Deserialize, Validate, Debug)]
pub struct Payload {
    #[validate(url)]
    pub url: String,
    #[validate(email)]
    pub email: String,
}

#[derive(Serialize)]
pub struct ResponseMsg {
    pub message: String,
}

pub async fn send_handler(
    insecure_ip: InsecureClientIp,
    Json(payload): Json<Payload>,
) -> (StatusCode, Json<ResponseMsg>) {
    let mut res = ResponseMsg {
        message: String::from(""),
    };

    match payload.validate() {
        Ok(_) => {
            tracing::info!("\n{:#?}", payload);
            tokio::spawn(async move {
                let pdf = services::pdf::gen_pdf(&payload.url)
                    .await
                    .expect("could not convert url to pdf");

                services::email::send_email(&pdf, &payload.email, &payload.url)
                    .await
                    .expect("could not email pdf");

                services::analytics::log_analytics(insecure_ip)
                    .await
                    .expect("could not log analytics");

                tracing::info!("completed job");
            });
            res.message = String::from("Success. Document is being sent!");
            (StatusCode::OK, Json(res))
        }
        Err(e) => match e.field_errors().keys().next() {
            Some(x) => {
                res.message = format!("Error. Invalid {}", x);
                (StatusCode::BAD_REQUEST, Json(res))
            }
            None => {
                res.message = String::from("Error. An error occured");
                (StatusCode::BAD_REQUEST, Json(res))
            }
        },
    }
}
