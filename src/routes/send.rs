use crate::services;
use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use tokio;
use validator::Validate;

#[derive(Deserialize, Validate)]
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

pub async fn send_handler(Json(payload): Json<Payload>) -> (StatusCode, Json<ResponseMsg>) {
    let mut res = ResponseMsg {
        message: String::from(""),
    };

    match payload.validate() {
        Ok(_) => {
            tokio::spawn(async move {
                let pdf = services::pdf::gen_pdf(&payload.url)
                    .await
                    .expect("could not convert url to pdf");
                services::email::send_email(&pdf, &payload.email, &payload.url)
                    .await
                    .expect("could not email pdf");
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
