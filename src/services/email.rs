extern crate slug;
use slug::slugify;

use reqwest;
use anyhow::Result;
use serde_json::json;
use base64::{Engine as _, engine::general_purpose};

pub async fn send_email(data:&Vec<u8>, email: &String, url: &String) -> Result<reqwest::Response, reqwest::Error>{
    let base64_data =  general_purpose::STANDARD.encode(data);

    let body = json!({
    "From": dotenv!("FROM_EMAIL"),
    "To": email,
    "Subject": "New Webpage Order Just Got Delivered!",
    "TextBody": "Hey there, here's your freshly baked webpage from send2kindle. Enjoy!",
    "Attachments": [
    {
    "Name": slugify(url)+".pdf",
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
