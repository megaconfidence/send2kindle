use anyhow::Result;
use headless_chrome::Browser;

pub async fn gen_pdf(url: &String) -> Result<Vec<u8>> {
    let browser = Browser::default()?;
    let tab = browser.new_tab()?;

    let pdf_data = tab 
        .navigate_to(url)?
        .wait_until_navigated()?
        .print_to_pdf(None)?;

    Ok(pdf_data)
}
