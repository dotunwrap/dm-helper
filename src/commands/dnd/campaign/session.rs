use crate::{
    responses,
    structs::Session,
    utils::{
        autocompletes::autocomplete_campaign,
        checks,
        date::{get_long_date_week_day_timestamp, is_date_format_valid, is_date_in_future},
        db,
        guilds::get_guild_id,
        responses::get_responses_for_session,
        sessions,
    },
    Context, Error,
};
use chrono::Local;
use mysql::prelude::*;
use mysql::*;
use poise::serenity_prelude as serenity;

pub mod response;

const STATUS_PENDING: i64 = 0;
const STATUS_CONFIRMED: i64 = 1;
const STATUS_CANCELLED: i64 = 2;

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

    let guild_id = get_guild_id(ctx).await;
    let created_date = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    if !is_date_format_valid(&scheduled_date) {
        return responses::failure(ctx, "Invalid date format.").await;
    }

    if !is_date_in_future(&scheduled_date) {
        return responses::failure(ctx, "Scheduled date must be in the future.").await;
    }

    db::get_db_conn(ctx).exec_drop(
        "INSERT INTO sessions (
                campaign_id,
                author_id,
                location,
                status,
                created_date,
                scheduled_date
            )
            SELECT id, :author_id, :location, :status, :created_date, :scheduled_date
            FROM campaigns 
            WHERE name = :campaign_name AND guild_id = :guild_id",
        params! {
            "author_id" => ctx.author().id.to_string(),
            "location" => location,
            "status" => 0,
            "created_date" => created_date,
            "scheduled_date" => scheduled_date,
            "campaign_name" => campaign,
            "guild_id" => guild_id.get()
        },
    )?;

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
    #[description = "The ID of the session to edit"] session_id: i64,
    #[description = "The new location"] location: String,
) -> Result<(), Error> {
    ctx.defer().await?;

    if !sessions::does_session_exist(ctx, session_id).await {
        return responses::failure(ctx, "Session not found.").await;
    }

    db::get_db_conn(ctx).exec_drop(
        "UPDATE sessions SET location = :location WHERE id = :session_id",
        params! {
            "location" => &location,
            "session_id" => session_id
        },
    )?;

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
    #[description = "The ID of the session to edit"] session_id: i64,
    #[description = "The new date (YYYY-MM-DD HH:MM)"] date: String,
) -> Result<(), Error> {
    ctx.defer().await?;

    if !sessions::does_session_exist(ctx, session_id).await {
        return responses::failure(ctx, "Session not found.").await;
    }

    if !is_date_format_valid(&date) {
        return responses::failure(ctx, "Invalid date format.").await;
    }

    if !is_date_in_future(&date) {
        return responses::failure(ctx, "Scheduled date must be in the future.").await;
    }

    db::get_db_conn(ctx).exec_drop(
        "UPDATE sessions SET scheduled_date = :date WHERE id = :session_id",
        params! {
            "date" => &date,
            "session_id" => session_id
        },
    )?;

    responses::success(
        ctx,
        &format!(
            "Date updated to {} for session ID {}",
            get_long_date_week_day_timestamp(&date).unwrap_or(date),
            session_id,
        ),
    )
    .await
}

/// Cancels a D&D session (DMs only)
#[poise::command(slash_command, check = "checks::dm_check")]
pub async fn cancel(
    ctx: Context<'_>,
    #[description = "The ID of the session to cancel"] session_id: i64,
) -> Result<(), Error> {
    ctx.defer().await?;

    if !sessions::does_session_exist(ctx, session_id).await {
        return responses::failure(ctx, "Session not found.").await;
    }

    db::get_db_conn(ctx).exec_drop(
        format!(
            "UPDATE sessions SET status = {} WHERE id = :session_id",
            STATUS_CANCELLED
        ),
        params! {
            session_id
        },
    )?;

    responses::success(ctx, "Session cancelled.").await
}

/// Deletes all D&D sessions for the guild (owner only)
#[poise::command(slash_command, owners_only)]
pub async fn clear_all(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;

    async fn clear_all_sessions(ctx: Context<'_>, _: ()) -> Result<(), Error> {
        let mut conn = db::get_db_conn(ctx);
        let guild_id = get_guild_id(ctx).await;

        conn.exec_drop(
            "DELETE FROM sessions
            WHERE campaign_id IN (
                SELECT id FROM campaigns WHERE guild_id = :guild_id
            )",
            params! { "guild_id" => guild_id.get() },
        )?;
        conn.exec_drop(
            "DELETE FROM responses
            WHERE session_id IN (
                SELECT id FROM sessions WHERE campaign_id IN (
                    SELECT id FROM campaigns WHERE guild_id = :guild_id
                )
            )",
            params! { "guild_id" => guild_id.get() },
        )?;

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
    campaign: Option<String>,
) -> Result<(), Error> {
    ctx.defer().await?;

    let guild_id = get_guild_id(ctx).await;
    let mut embeds: Vec<serenity::CreateEmbed> = vec![];
    let mut where_clause =
        String::from("WHERE s.scheduled_date > NOW() AND c.guild_id = :guild_id");
    let mut params = params! {
        "guild_id" => guild_id.get()
    };

    match campaign {
        Some(campaign) => {
            where_clause.push_str(" AND c.name = :campaign");
            params = params! {
                "guild_id" => guild_id.get(),
                campaign
            };
        }
        None => {}
    };

    db::get_db_conn(ctx).exec_map(
        format!(
            "SELECT s.id AS id,
                c.name AS campaign_name,
                s.location AS location,
                s.status AS status,
                DATE_FORMAT(s.created_date, '%Y-%m-%d %H:%i') AS created_date,
                DATE_FORMAT(s.scheduled_date, '%Y-%m-%d %H:%i') AS scheduled_date
                FROM sessions s
                INNER JOIN campaigns c
                ON s.campaign_id = c.id
                {}
                {}",
            where_clause, "ORDER BY scheduled_date ASC"
        ),
        params,
        |(id, campaign_name, location, status, created_date, scheduled_date): (
            i64,
            String,
            String,
            i64,
            String,
            String,
        )| {
            let mut going: Vec<String> = vec![];
            let mut not_going: Vec<String> = vec![];
            let scheduled_date =
                get_long_date_week_day_timestamp(&scheduled_date).unwrap_or(scheduled_date);

            get_responses_for_session(ctx, id)
                .into_iter()
                .for_each(|r| match r.response {
                    1 => going.push(format!("<@{}>", r.respondee_id)),
                    0 => not_going.push(format!("<@{}>", r.respondee_id)),
                    _ => {}
                });

            embeds.push(
                serenity::CreateEmbed::new()
                    .title(format!("{}", campaign_name))
                    .field("Location", location, true)
                    .field("Status", Session::translate_status(status), true)
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
                        id, created_date
                    ))),
            );
        },
    )?;

    responses::paginate_embeds(ctx, embeds).await
}

/// Sets the status of an existing session (DMs only)
#[poise::command(slash_command, check = "checks::dm_check")]
pub async fn set(
    ctx: Context<'_>,
    #[description = "The ID of the session to edit"] session_id: i64,
    #[description = "The status to set the session to"] status: StatusChoice,
) -> Result<(), Error> {
    ctx.defer().await?;

    if !sessions::does_session_exist(ctx, session_id).await {
        return responses::failure(ctx, "Session not found.").await;
    }

    let status = match status {
        StatusChoice::Pending => STATUS_PENDING,
        StatusChoice::Confirmed => STATUS_CONFIRMED,
        StatusChoice::Cancelled => STATUS_CANCELLED,
    };

    db::get_db_conn(ctx).exec_drop(
        "UPDATE sessions SET status = :status WHERE id = :session_id",
        params! {
            status,
            session_id
        },
    )?;

    responses::success(
        ctx,
        &format!("Status updated to {} for session ID {}", status, session_id),
    )
    .await
}
