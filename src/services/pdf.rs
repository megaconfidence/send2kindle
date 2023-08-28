use anyhow::Result;
use headless_chrome::{Browser, LaunchOptions};

pub async fn gen_pdf(url: &String) -> Result<Vec<u8>> {
    let mut args = Vec::new();
    args.push(std::ffi::OsStr::new("--no-sandbox"));
    args.push(std::ffi::OsStr::new("--headless=new"));
    args.push(std::ffi::OsStr::new("--disable-gpu"));

    let browser = Browser::new(LaunchOptions {
        args,
        headless: false,
        ..Default::default()
    })?;

    let tab = browser.new_tab()?;

    let pdf_data = tab
        .navigate_to(url)?
        .wait_until_navigated()?
        .print_to_pdf(None)?;

    Ok(pdf_data)
}
