use chrono::{Local, NaiveDateTime, TimeZone};

pub fn is_date_in_future(date: &str) -> bool {
    let current_date = Local::now();
    let parsed_date = NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M");

    if parsed_date.is_err() {
        return false;
    }

    let parsed_date = Local.from_local_datetime(&parsed_date.unwrap()).unwrap();

    parsed_date > current_date
}

pub fn is_date_format_valid(date: &str) -> bool {
    NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M").is_ok()
}
