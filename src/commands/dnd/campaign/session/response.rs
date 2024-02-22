use crate::utils::db;
use crate::{responses, Context, Error};
use mysql::params;
use mysql::prelude::*;

#[derive(poise::ChoiceParameter)]
enum ResponseChoice {
    Yes,
    No,
}

/// Responds to a D&D session
#[poise::command(slash_command)]
pub async fn respond(
    ctx: Context<'_>,
    #[description = "The ID of the session you're responding to"] session_id: i64,
    #[description = "Are you going?"] response: ResponseChoice,
) -> Result<(), Error> {
    let response = match response {
        ResponseChoice::Yes => 1,
        ResponseChoice::No => 0,
    };

    db::get_db_conn(ctx).exec_drop(
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
