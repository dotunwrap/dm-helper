use super::db;
use crate::{Context, structs::Response};
use mysql::*;
use mysql::prelude::*;

pub fn get_responses_for_session(ctx: Context<'_>, session_id: i64) -> Vec<Response> {
    db::get_db_conn(ctx) 
        .exec_map(
            "SELECT id, session_id, respondee_id, response, DATE_FORMAT(responded_date, '%Y-%m-%d %H:%i') AS responded_date
            FROM responses
            WHERE session_id = :session_id",
            params! { session_id },
            |(id, session_id, respondee_id, response, responded_date): (
                i64,
                i64,
                String,
                i64,
                String,
            )| Response {
                id,
                session_id,
                respondee_id,
                response,
                responded_date,
            },
        )
        .expect("Failed to get response information")
}
