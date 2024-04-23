use crate::models::NewResponse;
use crate::ops::{response_ops, session_ops};
use crate::utils::checks;
use crate::utils::id::user_id_to_i64;
use crate::{responses, Context, Error};

#[derive(poise::ChoiceParameter)]
enum ResponseChoice {
    Yes,
    No,
}

/// Responds to a D&D session as DM
#[poise::command(slash_command, check = "checks::dm_check")]
pub async fn dmrespond(
    ctx: Context<'_>,
    #[description = "The ID of the session you're responding to"] session_id: i32,
    #[description = "Are you going?"] response: ResponseChoice,
    #[description = "Who is going? Defaults to you."] respondee: Option<serenity::User,>,
) -> Result<(), Error> {
    if !session_ops::does_session_exist(ctx, session_id) {
        return responses::failure(ctx, "Session not found.").await;
    }

    let response = match response {
        ResponseChoice::Yes => 1,
        ResponseChoice::No => 0,
    };

    let respondee_id = match respondee {
        Some(respondee) => respondee.id,
        None => ctx.author().id,
    };

    let new_response = NewResponse {
        session_id,
        respondee_id: user_id_to_i64(respondee_id).await,
        response,
        responded_date: chrono::Utc::now().naive_utc(),
    };

    response_ops::create_response(ctx, new_response);

    responses::success(ctx, "Response recorded.").await
}
