pub async fn round_to_nearest_10(num: i64) -> i64 {
    ((num as f64 / 10.0).round() * 10.0) as i64
}
