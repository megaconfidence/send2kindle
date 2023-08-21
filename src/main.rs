use reqwest;
use anyhow::Result;
use serde_json::json;
use headless_chrome::Browser;
use base64::{Engine as _, engine::general_purpose};


use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize};
use std::net::SocketAddr;


#[macro_use]
extern crate dotenv_codegen;

async fn send_email(data:&Vec<u8>, email: &String) -> Result<reqwest::Response, reqwest::Error>{
    let base64_data =  general_purpose::STANDARD.encode(data);

    let body = json!({
    "From": "confidence@confidence.sh",
    "To": email,
    "Subject": "json",
    "TextBody": "json",
    "Attachments": [
    {
    "Name": "web.pdf",
    "Content": base64_data,
    "ContentType": "application/pdf"
    }
    ]
    });



    let client = reqwest::Client::new();
    let res = client.post("https://api.postmarkapp.com/email")
        .header("Content-Type", "application/json")
        .header("X-Postmark-Server-Token", dotenv!("POSTMARK_TOKEN"))
        .json(&body)
        .send()
    .await;

    return res;
}


async fn gen_pdf(url: &String) -> Result<Vec<u8>> {

    let browser = Browser::default()?;
    let tab = browser.new_tab()?;

    let wikidata = tab 
        .navigate_to(url)?
        .wait_until_navigated()?
        .print_to_pdf(None)?;

    // std::fs::write("wiki.pdf", &wikidata)?;
    // println!("PDF successfully created from internet web page.");

    Ok(wikidata)
}

#[derive(Deserialize)]
struct Payload {
    url: String,
    email: String,
}

async fn send(Json(payload): Json<Payload>) -> (StatusCode, String) {
    let pdf = gen_pdf(&payload.url).await;
    let _res = send_email(&pdf.unwrap(), &payload.email).await;  

    // println!("{:?}", _res);

    (StatusCode::OK, "pdf sent".to_string())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(|| async { "server online" }))
        .route("/send", post(send));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

}
