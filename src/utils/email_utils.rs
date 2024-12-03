use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use serde::{Deserialize, Serialize};
use std::env;
use dotenv::dotenv;
use log::{info, error};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct EmailService {
    pub to:String,
    pub from: String,
    pub reply: String,
    pub subject: String,
    pub body: String,
}

pub fn send_email(to: String, from: String, reply: String, subject: String, body: String) {

    dotenv().ok();
    let HOST = env::var("MAIL_HOST").expect("");
    let MAIL_USERNAME = env::var("MAIL_USERNAME").expect("");
    let MAIL_PASSWORD = env::var("MAIL_PASSWORD").expect("");
    let MAIL_FROM = env::var("MAIL_FROM").expect("");

    let email = Message::builder()
        .from(from.parse().unwrap())
        .reply_to(reply.parse().unwrap())
        .to(to.parse().unwrap())
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(body)
        .unwrap();

    info!("Mail {:?}",MAIL_USERNAME);

    let creds = Credentials::new(MAIL_USERNAME.to_owned(), MAIL_PASSWORD.to_owned());
    let mailer = SmtpTransport::relay("mail.zimacani.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {e:?}"),
    }
}
