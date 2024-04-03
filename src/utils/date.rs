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

pub fn get_relative_timestamp(date: &str) -> Result<String, chrono::ParseError> {
    let timestamp = get_unix_timestamp(date)?;

    Ok(format!("<t:{}:R>", timestamp))
}

pub fn get_short_time_timestamp(date: &str) -> Result<String, chrono::ParseError> {
    let timestamp = get_unix_timestamp(date)?;

    Ok(format!("<t:{}:t>", timestamp))
}

pub fn get_long_time_timestamp(date: &str) -> Result<String, chrono::ParseError> {
    let timestamp = get_unix_timestamp(date)?;

    Ok(format!("<t:{}:T>", timestamp))
}

pub fn get_short_date_timestamp(date: &str) -> Result<String, chrono::ParseError> {
    let timestamp = get_unix_timestamp(date)?;

    Ok(format!("<t:{}:d>", timestamp))
}

pub fn get_long_date_timestamp(date: &str) -> Result<String, chrono::ParseError> {
    let timestamp = get_unix_timestamp(date)?;

    Ok(format!("<t:{}:D>", timestamp))
}

pub fn get_long_date_short_time_timestamp(date: &str) -> Result<String, chrono::ParseError> {
    let timestamp = get_unix_timestamp(date)?;

    Ok(format!("<t:{}:f>", timestamp))
}

pub fn get_long_date_week_day_timestamp(date: &str) -> Result<String, chrono::ParseError> {
    let timestamp = get_unix_timestamp(date)?;

    Ok(format!("<t:{}:F>", timestamp))
}

pub fn get_unix_timestamp(date: &str) -> Result<i64, chrono::ParseError> {
    match NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M") {
        Ok(parsed_date) => Ok(parsed_date.and_utc().timestamp()),
        Err(e) => Err(e),
    }
}
