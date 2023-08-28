extern crate slug;
use slug::slugify;

// use anyhow::Result;
use mail_builder::MessageBuilder;
use mail_send::SmtpClientBuilder;

pub async fn send_email(
    data: &Vec<u8>,
    to_email: &String,
    url: &String,
) -> Result<(), mail_send::Error> {
    let file_name = slugify(url) + ".pdf";
    let smtp_server = std::env::var("SMTP_SERVER").expect("provide $SMTP_SERVER");
    let smtp_username = std::env::var("SMTP_USERNAME").expect("provide $SMTP_USERNAME");
    let smtp_password = std::env::var("SMTP_PASSWORD").expect("provide $SMTP_PASSWORD");
    let smtp_from_email = std::env::var("SMTP_FROM_EMAIL").expect("provide $SMTP_FROM_EMAIL");
    let smtp_port = std::env::var("SMTP_PORT")
        .expect("provide $SMTP_PORT")
        .parse()
        .unwrap();

    let message = MessageBuilder::new()
        .from(smtp_from_email)
        .to(to_email.to_string())
        .subject("New Webpage Order Just Got Delivered!")
        .text_body("Hey there, here's your freshly baked webpage from send2kindle. Enjoy!")
        .attachment("application/pdf", file_name, data.to_owned());

    return SmtpClientBuilder::new(smtp_server, smtp_port)
        .implicit_tls(false)
        .credentials((smtp_username, smtp_password))
        .connect()
        .await
        .unwrap()
        .send(message)
        .await;
}
