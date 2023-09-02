extern crate chrono;

use chrono::{Utc, DateTime, TimeZone};

fn main() {
    println!("Hello, Days2You!");
    get_days_to();
}

fn get_days_to() {
    let start_date = Utc.with_ymd_and_hms(2023, 8, 31, 0, 0, 0).unwrap();
    let end_date = Utc.with_ymd_and_hms(2023, 12, 8, 0, 0, 0).unwrap();

    let duration = get_days_between(start_date, end_date);

    println!("Ci sono {} giorni tra {} e {}", duration, start_date, end_date);
}

fn get_days_between(start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> i64 {
    return (end_date - start_date).num_days();
}
