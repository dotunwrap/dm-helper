use crate::{
    responses,
    structs::Session,
    utils::{
        autocompletes::autocomplete_campaign, checks, db, guilds::get_guild_id,
        responses::get_responses_for_session, sessions,
    },
    Context, Error,
};
use chrono::{Local, NaiveDateTime, TimeZone};
use mysql::params;
use mysql::prelude::*;
use poise::serenity_prelude as serenity;

pub mod response;

const _STATUS_PENDING: i64 = 0;
const _STATUS_CONFIRMED: i64 = 1;
const STATUS_CANCELLED: i64 = 2;

enum StatusChoice {
    Pending,
    Confirmed,
    Cancelled,
}

/// D&D Sessions (subcommand required)
#[poise::command(
    slash_command,
    subcommands("create", "cancel", "clear_all", "list", "response::respond"),
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
    #[description = "Campaign to attribute the session to"]
    #[autocomplete = autocomplete_campaign]
    campaign: String,
    #[description = "Where to meet"] location: String,
    #[description = "Date and time of the session (YYYY-MM-DD HH:MM)"] scheduled_date: String,
) -> Result<(), Error> {
    ctx.defer().await?;

    let guild_id = get_guild_id(ctx).await;
    let created_date = Local::now();
    let scheduled_date = NaiveDateTime::parse_from_str(&scheduled_date, "%Y-%m-%d %H:%M");

    if scheduled_date.is_err() {
        return responses::failure(ctx, "Invalid date format.").await;
    }

    let scheduled_date = Local.from_local_datetime(&scheduled_date.unwrap()).unwrap();

    if scheduled_date < created_date {
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
            "created_date" => created_date.format("%Y-%m-%d %H:%M:%S").to_string(),
            "scheduled_date" => scheduled_date.format("%Y-%m-%d %H:%M:%S").to_string(),
            "campaign_name" => campaign,
            "guild_id" => guild_id.get()
        },
    )?;

    responses::success(ctx, "Session created.").await
}

/// Cancels a D&D session (DMs only)
#[poise::command(slash_command, check = "checks::dm_check")]
pub async fn cancel(
    ctx: Context<'_>,
    #[description = "Session ID"] session_id: i64,
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
            where_clause.push_str(" AND s.campaign_id = :campaign");
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
                    .field("Date/Time", scheduled_date, true)
                    .field("Location", location, true)
                    .field("Status", Session::translate_status(status), true)
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
