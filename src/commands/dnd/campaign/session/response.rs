use crate::utils::db;
use crate::{responses, Context, Error};
use mysql::prelude::*;
use mysql::*;

pub struct Response {
    pub id: i64,
    pub session_id: i64,
    pub respondee_id: String,
    pub response: i64,
    pub responded_date: String,
}

#[derive(poise::ChoiceParameter)]
enum ResponseChoice {
    Yes,
    No,
}

/// Responds to a D&D session
#[poise::command(prefix_command, slash_command)]
pub async fn respond(
    ctx: Context<'_>,
    #[description = "The ID of the session you're responding to"] session_id: i64,
    #[description = "Are you going?"] response: ResponseChoice,
) -> Result<(), Error> {
    let response = match response {
        ResponseChoice::Yes => 1,
        ResponseChoice::No => 0,
    };

    db::get_db_conn(ctx)
        .exec_drop(
            "INSERT INTO responses (
                session_id,
                respondee_id,
                response,
                responded_date
            ) VALUES (
                :session_id,
                :respondee_id,
                :response,
                NOW()
            ) ON DUPLICATE KEY UPDATE response = :response",
            params! {
                "session_id" => session_id,
                "respondee_id" => ctx.author().id.to_string(),
                "response" => response as i64
            },
        )?;

    responses::success(ctx, "Response recorded.").await
}

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
