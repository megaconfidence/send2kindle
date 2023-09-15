use anyhow::Result;
use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide::cdp::browser_protocol::page::PrintToPdfParams;
use futures::StreamExt;

pub async fn gen_pdf(url: &String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let pdfOptions = PrintToPdfParams {
        margin_top: Some(0.),
        margin_left: Some(0.),
        margin_right: Some(0.),
        margin_bottom: Some(0.),
        print_background: Some(true),
        ..Default::default()
    };

    let (mut browser, mut handler) = Browser::launch(
        BrowserConfig::builder()
            .no_sandbox()
            .with_head()
            // .launch_timeout(std::time::Duration::new(3600, 0))
            .incognito()
            // .request_timeout(std::time::Duration::new(3600, 0))
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

    page.evaluate(
        r#"(() => {
      const scroll = resolve => {
        const scrollingElement = document.scrollingElement || document.body;
        scrollingElement.scrollBy(0, 100);
        const tID = setTimeout(() => {
          scroll(resolve);
        }, 100);

        if (scrollingElement.scrollTop / scrollingElement.scrollHeight >= 0.9) {
          clearTimeout(tID);
          resolve();
        }
      };
      return new Promise(resolve => {
        scroll(resolve);
      });
    })() "#,
    )
    .await?;

    page.evaluate(
        r#"(() => {
        Promise.all(Array.from(document.images).map(img => {
            if (img.complete)
                return Promise.resolve(img.naturalHeight !== 0);
            return new Promise(resolve => {
                img.addEventListener('load', () => resolve(true));
                img.addEventListener('error', () => resolve(false));
            });
        })).then(results => {
            if (results.every(res => res))
                console.log('all images loaded successfully');
            else
                console.log('some images failed to load, all finished loading');
        });
    })()"#,
    )
    .await?;

    let pdf = page.pdf(pdfOptions).await?;
    // let pdf = page
    //     .save_pdf(pdfOptions, std::path::Path::new("./web.pdf"))
    //     .await?;

    browser.close().await?;
    handle.await;

    Ok(pdf)
}
