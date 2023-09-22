use anyhow::Result;
use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide::cdp::browser_protocol::page::PrintToPdfParams;
use futures::StreamExt;
use nanoid::nanoid;
use std::path::Path;

pub async fn gen_pdf(url: &String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let pdf_options = PrintToPdfParams {
        margin_top: Some(0.),
        margin_left: Some(0.),
        margin_right: Some(0.),
        margin_bottom: Some(0.),
        print_background: Some(true),
        ..Default::default()
    };
    let image_js = include_str!("./browser/image.js");
    let scroll_js = include_str!("./browser/scroll.js");

    let (mut browser, mut handler) = Browser::launch(
        BrowserConfig::builder()
            .arg("--no-sandbox")
            .arg("--headless=new")
            .arg("--disable-gpu")
            .build()?,
    )
    .await?;
    let handle = tokio::spawn(async move {
        while let Some(h) = handler.next().await {
            if h.is_err() {
                break;
            }
        }
    });
    let page = browser.new_page(url).await?;

    page.wait_for_navigation().await?;
    page.evaluate(scroll_js).await?;
    page.evaluate(image_js).await?;

    let job_id = nanoid!();
    let file_name = format!("webpage_{}_.pdf", job_id);
    let pdf_path = Path::new(&file_name);

    page.save_pdf(pdf_options, pdf_path).await?;
    let pdf = send2kindle::compress_pdf(pdf_path);
    send2kindle::clean_files(&job_id)?;

    browser.close().await?;
    handle.await?;

    Ok(pdf)
}

