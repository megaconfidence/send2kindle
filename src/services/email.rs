extern crate slug;
use slug::slugify;

use anyhow::Result;
use base64::{Engine as _, engine::general_purpose};

use lettre::message::{Mailbox, header::ContentType, Attachment, MultiPart, SinglePart};
use lettre::Address;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub async fn send_email(data:&Vec<u8>, to_email: &String, url: &String) -> Result<()> {
    let base64_data =  general_purpose::STANDARD.encode(data);
    let smtp_from_email = std::env::var("SMTP_FROM_EMAIL").expect("provide $SMTP_FROM_EMAIL");
    let smtp_username = std::env::var("SMTP_USERNAME").expect("provide $SMTP_USERNAME");
    let smtp_password = std::env::var("SMTP_PASSWORD").expect("provide $SMTP_PASSWORD");
    let smtp_server = std::env::var("SMTP_SERVER").expect("provide $SMTP_SERVER");
    let file_name = slugify(url)+".pdf";
    
    println!("1");
    let email = Message::builder()
        .from(Mailbox::new(None, smtp_from_email.parse::<Address>()?))
        .to(Mailbox::new(None, to_email.parse::<Address>()?))
        .subject("New Webpage Order Just Got Delivered!")
        .header(ContentType::TEXT_PLAIN)
        // .multipart(
        //     MultiPart::mixed()
        //         .singlepart(SinglePart::plain(String::from("Hey there, here's your freshly baked webpage from send2kindle. Enjoy!")))
        //         // .singlepart(Attachment::new(file_name).body(base64_data,  ContentType::parse("application/pdf").unwrap()))
        // )
        .body(String::from("Hey there, here's your freshly baked webpage from send2kindle. Enjoy!"))
        .unwrap();

    println!("2");
    let creds = Credentials::new(smtp_username, smtp_password);

    println!("3");
    let mailer = SmtpTransport::relay(&smtp_server)
        .unwrap()
        .credentials(creds)
        .port(25)
        .build();

    println!("4");
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }

    println!("5");
    Ok(())
}
