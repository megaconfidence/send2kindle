use std::fs;
use anyhow::Result;
use headless_chrome::Browser;


async fn send_email() {
}

#[tokio::main]
async fn main() -> Result<()>{
    let browser = Browser::default()?;
    let tab = browser.new_tab()?;

    let wikidata = tab 
        .navigate_to("http://example.com/")?
        .wait_until_navigated()?
        .print_to_pdf(None)?;



    // println!("{:?}", wikidata);
    // println!("{}", std::str::from_utf8(&wikidata).unwrap() );
    fs::write("wiki.pdf", wikidata)?;
    println!("PDF successfully created from internet web page.");

    let res = send_email().await;  
    println!("{:?}", res);

    Ok(())
}
