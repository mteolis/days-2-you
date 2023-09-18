extern crate chrono;
extern crate lettre;

use std::fs;
use chrono::{Utc, DateTime, TimeZone};
use lettre::{Message, SmtpTransport, Transport, transport::smtp::authentication::Credentials};
// use lettre::message::{header::ContentType, Message};

fn main() {
    println!("Hello, Days2You!");
    // build_message();
    // send_message(build_message());
    get_days_to();
    // read_credentials();
    let message = build_message();

    let (username, password) = read_credentials();

    let credentials = Credentials::new(username.to_string(), password.to_string());

    // TODO: set up SMTP server
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(credentials)
        .build();

    match mailer.send(&message) {
        Ok(_) => println!("Email sent successfully"),
        Err(e) => println!("Could not send email: {:?}", e),
    }
    // println!("Username: {}", username);
    // println!("Password: {}", password);
}

fn read_credentials() -> (String, String) {
    let filename = "credentials";
    println!("Reading from {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();
    let username: &str = lines[0].split("=").collect::<Vec<_>>()[1];
    let password: &str = lines[1].split("=").collect::<Vec<_>>()[1];

    return (username.trim().to_string(), password.trim().to_string());
}

fn get_days_to() {
    let start_date: DateTime<Utc> = Utc::now();
    let end_date: DateTime<Utc> = Utc.with_ymd_and_hms(2023, 12, 8, 0, 0, 0).unwrap();

    let duration = get_days_between(start_date, end_date);

    println!("Ci sono {} giorni tra {} e {}", duration, start_date, end_date);
}

fn get_days_between(start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> i64 {
    return (end_date - start_date).num_days();
}

fn build_message() -> Message {

    println!("Message built:\n{}", std::str::from_utf8(&message.formatted()).unwrap());

    return message;
}

// fn send_message(message: Message) {
//     let sender = SendmailTransport::new();
// }
