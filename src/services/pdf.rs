use anyhow::Result;
use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide::cdp::browser_protocol::page::PrintToPdfParams;
use futures::StreamExt;
use nanoid::nanoid;
use std::path::Path;

pub async fn gen_pdf(url: &String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    tracing::info!("launching browser");
    let (mut browser, mut handler) = Browser::launch(
        BrowserConfig::builder()
            .arg("--no-sandbox")
            .arg("--headless=new")
            .arg("--disable-gpu")
            // .with_head()
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

    tracing::info!("navigating to url");
    let page = browser.new_page(url).await?;
    page.wait_for_navigation().await?;

    let image_js = include_str!("./browser/image.js");
    let scroll_js = include_str!("./browser/scroll.js");

    page.evaluate(scroll_js).await?;
    page.evaluate(image_js).await?;

    let job_id = nanoid!();
    let file_name = format!("webpage_{}_.pdf", job_id);
    let file_path = Path::new(&file_name);

    tracing::info!("converting to pdf");
    let pdf_options = PrintToPdfParams {
        margin_top: Some(0.),
        margin_left: Some(0.),
        margin_right: Some(0.),
        margin_bottom: Some(0.),
        print_background: Some(true),
        ..Default::default()
    };
    page.save_pdf(pdf_options, file_path).await?;

    tracing::info!("compressing pdf");
    let pdf = send2kindle::compress_pdf(file_path);
    send2kindle::clean_files(&job_id)?;

    tracing::info!("shuttiing down browser");
    browser.close().await?;
    browser.wait().await?;

    handle.await?; //driver to handle all browser actions. Must be called last

    Ok(pdf)
}
