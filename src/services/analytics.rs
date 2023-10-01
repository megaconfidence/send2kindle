use axum_client_ip::InsecureClientIp;
use reqwest::Error;
use std::collections::HashMap;

pub async fn log_analytics(ip: InsecureClientIp) -> Result<(), Error> {
    let app_domain = std::env::var("APP_DOMAIN").expect("provide $APP_DOMAIN");
    let plausible_domain = std::env::var("PLAUSIBLE_DOMAIN").expect("provide $PLAUSIBLE_DOMAIN");

    let url = format!("{app_domain}/send");
    let domain = app_domain.replace("https://", "");
    let user_agent =  "Mozilla/5.0 (X11; U; Linux i686; en; rv:1.8.1.12) Gecko/20080208 (Debian-1.8.1.12-2) Epiphany/2.20";

    let mut body = HashMap::new();
    body.insert("name", "pageview");
    body.insert("url", url.as_str());
    body.insert("domain", domain.as_str());

    let client = reqwest::Client::new();
    let response = client
        .post(format!("{plausible_domain}/api/event"))
        .header("User-Agent", user_agent)
        .header("X-Forwarded-For", ip.0.to_string())
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?;

    tracing::info!("logging analytics: {}", response.text().await?);
    Ok(())
}
