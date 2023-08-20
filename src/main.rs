use std::fs;
use reqwest;
use anyhow::Result;
use serde_json::json;
use headless_chrome::Browser;
use base64::{Engine as _, engine::general_purpose};

#[macro_use]
extern crate dotenv_codegen;

async fn send_email(data:&Vec<u8>) -> Result<reqwest::Response, reqwest::Error>{
    let base64Data =  general_purpose::STANDARD.encode(data);

    let body = json!({
        "From": "confidence@confidence.sh",
        "To": "cokoghenun@gmail.com",
        "Subject": "json",
        "TextBody": "json",
        "Attachments": [
            {
                "Name": "web.pdf",
                "Content": base64Data,
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

#[tokio::main]
async fn main() -> Result<()>{
    let browser = Browser::default()?;
    let tab = browser.new_tab()?;

    let wikidata = tab 
        .navigate_to("http://youtube.com/")?
        .wait_until_navigated()?
        .print_to_pdf(None)?;



    // println!("{:?}", wikidata);
    // println!("{}", std::str::from_utf8(&wikidata).unwrap() );
    fs::write("wiki.pdf", &wikidata)?;
    println!("PDF successfully created from internet web page.");

    let res = send_email(&wikidata).await;  
    println!("{:?}", res);

    Ok(())
}
