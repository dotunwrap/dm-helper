use crate::{
    models::{NewSession, Session},
    ops::{campaign_ops, response_ops, session_ops},
    responses,
    utils::{
        autocompletes::autocomplete_campaign,
        checks,
        date::{is_date_format_valid, is_date_in_future},
        guilds::get_guild_id,
        id::{guild_id_to_i64, user_id_to_i64},
    },
    Context, Error,
};
use diesel::prelude::*;
use poise::serenity_prelude as serenity;

pub mod response;

const STATUS_PENDING: i16 = 0;
const STATUS_CONFIRMED: i16 = 1;
const STATUS_CANCELLED: i16 = 2;

#[derive(poise::ChoiceParameter)]
enum StatusChoice {
    Pending,
    Confirmed,
    Cancelled,
}

/// D&D Sessions (subcommand required)
#[poise::command(
    slash_command,
    subcommands(
        "create",
        "edit",
        "cancel",
        "clear_all",
        "list",
        "set",
        "response::respond"
    ),
    subcommand_required,
    check = "checks::dnd_check",
    category = "D&D"
)]
pub async fn session(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Creates a new D&D session (DMs only)
///
/// The scheduled date must be in the future
#[poise::command(slash_command, check = "checks::dm_check")]
pub async fn create(
    ctx: Context<'_>,
    #[autocomplete = autocomplete_campaign]
    #[description = "Campaign to attribute the session to"]
    campaign: String,
    #[description = "Where to meet"] location: String,
    #[description = "Date and time of the session (YYYY-MM-DD HH:MM)"] scheduled_date: String,
) -> Result<(), Error> {
    ctx.defer().await?;

    let guild_id = guild_id_to_i64(get_guild_id(ctx).await).await;
    let created_date = chrono::Utc::now().naive_utc();

    if !is_date_format_valid(&scheduled_date) {
        return responses::failure(ctx, "Invalid date format.").await;
    }

    if !is_date_in_future(&scheduled_date) {
        return responses::failure(ctx, "Scheduled date must be in the future.").await;
    }

    if !campaign_ops::does_campaign_exist(ctx, &campaign, guild_id) {
        return responses::failure(ctx, "Campaign not found.").await;
    }

    let scheduled_date =
        Some(chrono::NaiveDateTime::parse_from_str(&scheduled_date, "%Y-%m-%d %H:%M").unwrap());

    let campaign_id = campaign_ops::get_id_from_name(ctx, &campaign, guild_id).unwrap();

    let new_session = NewSession {
        campaign_id,
        author_id: user_id_to_i64(ctx.author().id).await,
        location: Some(&location),
        status: STATUS_PENDING,
        created_date,
        scheduled_date,
    };

    session_ops::create_session(ctx, new_session);

    responses::success(ctx, "Session created.").await
}

/// Edits and existing session (DMs only)
#[poise::command(
    slash_command,
    subcommands("location", "date"),
    subcommand_required,
    check = "checks::dm_check"
)]
pub async fn edit(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Edits the location of an existing session (DMs only)
#[poise::command(slash_command)]
pub async fn location(
    ctx: Context<'_>,
    #[description = "The ID of the session to edit"] session_id: i32,
    #[description = "The new location"] location: String,
) -> Result<(), Error> {
    ctx.defer().await?;

    if !session_ops::does_session_exist(ctx, session_id) {
        return responses::failure(ctx, "Session not found.").await;
    }

    let updated_session = Session {
        id: session_id,
        location: Some(location.clone()),
        ..session_ops::get_session(ctx, session_id).unwrap()
    };

    session_ops::update_session(ctx, updated_session);

    responses::success(
        ctx,
        &format!(
            "Location updated to {} for session ID {}",
            location, session_id,
        ),
    )
    .await
}

/// Edits the date of an existing session (DMs only)
#[poise::command(slash_command)]
pub async fn date(
    ctx: Context<'_>,
    #[description = "The ID of the session to edit"] session_id: i32,
    #[description = "The new date (YYYY-MM-DD HH:MM)"] date: String,
) -> Result<(), Error> {
    ctx.defer().await?;

    if !session_ops::does_session_exist(ctx, session_id) {
        return responses::failure(ctx, "Session not found.").await;
    }

    if !is_date_format_valid(&date) {
        return responses::failure(ctx, "Invalid date format.").await;
    }

    if !is_date_in_future(&date) {
        return responses::failure(ctx, "Scheduled date must be in the future.").await;
    }

    let date = chrono::NaiveDateTime::parse_from_str(&date, "%Y-%m-%d %H:%M").unwrap();

    let updated_session = Session {
        id: session_id,
        scheduled_date: Some(date),
        ..session_ops::get_session(ctx, session_id).unwrap()
    };

    session_ops::update_session(ctx, updated_session);

    responses::success(
        ctx,
        &format!("Date updated to {} for session ID {}", date, session_id,),
    )
    .await
}

/// Cancels a D&D session (DMs only)
#[poise::command(slash_command, check = "checks::dm_check")]
pub async fn cancel(
    ctx: Context<'_>,
    #[description = "The ID of the session to cancel"] session_id: i32,
) -> Result<(), Error> {
    ctx.defer().await?;

    if !session_ops::does_session_exist(ctx, session_id) {
        return responses::failure(ctx, "Session not found.").await;
    }

    let updated_session = Session {
        id: session_id,
        status: STATUS_CANCELLED,
        ..session_ops::get_session(ctx, session_id).unwrap()
    };

    session_ops::update_session(ctx, updated_session);

    responses::success(ctx, "Session cancelled.").await
}

/// Deletes all D&D sessions for the guild (owner only)
#[poise::command(slash_command, owners_only)]
pub async fn clear_all(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;

    async fn clear_all_sessions(ctx: Context<'_>, _: ()) -> Result<(), Error> {
        let guild_id = guild_id_to_i64(get_guild_id(ctx).await).await;

        session_ops::bulk_cancel_sessions(ctx, guild_id);

        responses::success(ctx, "All sessions deleted.").await
    }

    responses::verify_command(ctx, &clear_all_sessions, ()).await
}

/// Lists all D&D sessions
#[poise::command(slash_command)]
pub async fn list(
    ctx: Context<'_>,
    #[autocomplete = autocomplete_campaign]
    #[description = "Campaign"]
    campaign: String,
) -> Result<(), Error> {
    ctx.defer().await?;

    let guild_id = guild_id_to_i64(get_guild_id(ctx).await).await;
    let mut embeds: Vec<serenity::CreateEmbed> = vec![];
    let conn = &mut crate::utils::db::get_conn(ctx);

    if !campaign_ops::does_campaign_exist(ctx, &campaign, guild_id) {
        return responses::failure(ctx, "Campaign not found.").await;
    }

    let campaign_id = campaign_ops::get_id_from_name(ctx, &campaign, guild_id).unwrap();

    let results: Option<Vec<(Session, String)>> = {
        use crate::schema::campaigns;
        use crate::schema::sessions;

        sessions::table
            .inner_join(campaigns::table)
            .filter(campaigns::id.eq(campaign_id))
            .filter(sessions::scheduled_date.ge(chrono::Utc::now().naive_utc()))
            .order_by(sessions::scheduled_date)
            .select((sessions::all_columns, campaigns::name))
            .load(conn)
            .ok()
    };

    if results.is_none() {
        return responses::failure(ctx, "No sessions found.").await;
    }

    results.unwrap().into_iter().for_each(|result| {
        let session = result.0;
        let campaign_name = result.1;
        let mut going: Vec<String> = vec![];
        let mut not_going: Vec<String> = vec![];

        if let Some(responses) = response_ops::get_responses_for_session(ctx, session.id) {
            responses.into_iter().for_each(|r| {
                if r.response == 1 {
                    going.push(format!("<@{}>", r.respondee_id));
                } else if r.response == 0 {
                    not_going.push(format!("<@{}>", r.respondee_id));
                }
            });
        }

        let status = match session.status {
            0 => String::from("Pending"),
            1 => String::from("Confirmed"),
            2 => String::from("Cancelled"),
            _ => String::from("Unknown"),
        };

        let location = match session.location {
            Some(location) => location,
            None => "None".to_string(),
        };

        let scheduled_date = session
            .scheduled_date
            .unwrap()
            .format("%Y-%m-%d %H:%M")
            .to_string();

        embeds.push(
            serenity::CreateEmbed::default()
                .title(format!("{}", campaign_name))
                .field("Location", location, true)
                .field("Status", status, true)
                .field("Date/Time", scheduled_date, false)
                .field(
                    "Going",
                    match going.is_empty() {
                        true => "None".to_string(),
                        false => going.join(", "),
                    },
                    false,
                )
                .field(
                    "Not Going",
                    match not_going.is_empty() {
                        true => "None".to_string(),
                        false => not_going.join(", "),
                    },
                    false,
                )
                .footer(serenity::CreateEmbedFooter::new(format!(
                    "Session ID: {} | Created at {}",
                    session.id, session.created_date
                ))),
        )
    });

    responses::paginate_embeds(ctx, embeds).await
}

/// Sets the status of an existing session (DMs only)
#[poise::command(slash_command, check = "checks::dm_check")]
pub async fn set(
    ctx: Context<'_>,
    #[description = "The ID of the session to edit"] session_id: i32,
    #[description = "The status to set the session to"] status: StatusChoice,
) -> Result<(), Error> {
    ctx.defer().await?;

    if !session_ops::does_session_exist(ctx, session_id) {
        return responses::failure(ctx, "Session not found.").await;
    }

    let status = match status {
        StatusChoice::Pending => STATUS_PENDING,
        StatusChoice::Confirmed => STATUS_CONFIRMED,
        StatusChoice::Cancelled => STATUS_CANCELLED,
    };

    let updated_session = Session {
        id: session_id,
        status,
        ..session_ops::get_session(ctx, session_id).unwrap()
    };

    session_ops::update_session(ctx, updated_session);

    responses::success(
        ctx,
        &format!("Status updated to {} for session ID {}", status, session_id),
    )
    .await
}
